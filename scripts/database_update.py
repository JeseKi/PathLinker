import os
import platform
import sqlite3
import pandas as pd
import uuid
from pathlib import Path
from subprocess import PIPE, run
import getpass
from datetime import datetime
import time
import sys

error_paths = []

def event_time_log(event:str, isdatatime: bool = True):
    """
    记录事件和其发生的时间
    :param event: 事件
    :param isdatatime: 是否为系统时间
    :return:
    """
    if isdatatime:
        now_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    else : now_time = time.time()
    with open('log.txt', 'a', encoding='utf-8') as f:
        f.write(f"[INFO]:{event} \n[TIME] {now_time}\n")

def get_data_local_dir():
    if platform.system() == "Windows":
        return Path(os.environ['LOCALAPPDATA'])
    elif platform.system() == "Linux":
        return Path.home() / ".local" / "share"
    else:
        raise OSError("不支持的操作系统")

def get_file_system_path(path):
    if platform.system() == "Linux":
        result = run(["df", path], stdout=PIPE, stderr=PIPE, text=True)
        if result.returncode == 0:
            return result.stdout.splitlines()[1].split()[5]
        else:
            raise Exception("无法获取文件系统路径")
    elif platform.system() == "Windows":
        return path.drive
    else:
        raise OSError("不支持的操作系统")

def create_hard_link(source, link):
    try:
        os.link(source, link)
    except OSError as e:
        print(f"无法创建硬链接: {e}")

def build_hard_link_path(original_path, file_name, user_name):
    short_uuid = str(uuid.uuid4())[:5]
    extension = original_path.suffix.lstrip('.')
    new_file_name = f"{file_name}.{short_uuid}.{extension}"
    
    if platform.system() == "Linux":
        file_system_path = get_file_system_path(original_path)
        hard_link_path = Path(file_system_path) / user_name / ".pathlinker" / new_file_name
    elif platform.system() == "Windows":
        drive_letter = original_path.drive  # 应该已经包含冒号（如 C:）
        root_path = Path(f"{drive_letter}\\")  # 构造绝对路径的根，如 C:\
        if drive_letter.upper() == 'C:\\':
            hard_link_path = root_path / "Users" / user_name / ".pathlinker" / new_file_name
        else:
            hard_link_path = root_path / user_name / ".pathlinker" / new_file_name
    else:
        raise OSError("不支持的操作系统")

    return hard_link_path

# 确定数据库的路径
db_path = get_data_local_dir() / "pathlinker" / "pathlinker.db"

# 连接到数据库并读取数据
conn = sqlite3.connect(str(db_path))
try:
    df = pd.read_sql_query("SELECT id, file_name, path, url FROM mapping", conn)
except:
    try:
        pd.read_sql_query("SELECT id, file_name, origin_path, url , hard_link FROM mapping", conn)
        print("(｡•ˇ‸ˇ•｡)数据库已经迁移完成，不需要重复运行程序哦～")
        input("按回车键退出...")
        sys.exit(0)
    except Exception as e:
        print(f"(|||❛︵❛.)读取数据库失败: {e}")
        event_time_log(f"读取数据库失败: {e}")
        input("按回车键退出...")
        sys.exit(1)
        

# 获取当前用户的登录名
user_name = getpass.getuser()
print(f"用户名: {user_name}")
event_time_log(f"获取到用户名: {user_name}")
print(f"平台: {platform.system()}")
event_time_log(f"平台: {platform.system()}")

# 生成硬链接并更新DataFrame
for index, row in df.iterrows():
    original_path = Path(row['path'])
    hard_link_path = build_hard_link_path(original_path, row['file_name'], user_name)
    df.at[index, 'hard_link'] = str(hard_link_path)

for index, row in df.iterrows():
    original_path = Path(row['path'])
    hard_link_path = build_hard_link_path(original_path, row['file_name'], user_name)
    
    # 检查硬链接目标目录是否存在，不存在则创建
    hard_link_path.parent.mkdir(parents=True, exist_ok=True)
    # 检查源文件是否存在
    if not original_path.exists():
        print(f"源文件不存在，无法创建硬链接: {original_path}")
        event_time_log(f"源文件不存在，无法创建硬链接: {original_path}")
        continue  # 源文件不存在则跳过当前循环的剩余部分

    print(f"尝试创建硬链接：{hard_link_path} -> {original_path}")
    
    # 创建硬链接
    try:
        create_hard_link(original_path, hard_link_path)
        df.at[index, 'hard_link'] = str(hard_link_path)
    except Exception as e:
        print(f"对于\"{hard_link_path} -> {original_path}\"的硬链接创建失败！")
        event_time_log(f"对于\"{hard_link_path} -> {original_path}\"的硬链接创建失败！错误如下:\n{e}")
        error_paths.append(original_path)

# 检查并更新列名
if 'path' in df.columns:
    df.rename(columns={'path': 'origin_path'}, inplace=True)

# 导出到CSV
csv_path = db_path.with_suffix('.csv')
df.to_csv(csv_path, index=False)

# 关闭数据库连接
conn.close()

if len(error_paths) > 0:
    event_time_log(f"硬链接创建出错的路径:\n{error_paths}\n请尝试联系软件作者、提交issue或将`log.txt`发送到作者邮箱中。")

new_df = pd.read_csv(csv_path)
new_db_path = db_path.with_suffix('.new.db')
# 创建新的数据库连接和表结构
conn = sqlite3.connect(new_db_path)
df.to_sql('mapping', conn, if_exists='replace', index=False)

# 关闭新数据库连接
conn.close()

# 删除原数据库文件，并将新数据库文件重命名
db_path.unlink(missing_ok=True)  # 删除原数据库文件，如果文件不存在也不会引发错误
new_db_path.rename(db_path)  # 将新数据库文件重命名为原数据库文件的名称

print("  ˶╹ꇴ╹˶  数据库迁移完成！")
event_time_log("数据库迁移完成。")
input("按回车键退出...")