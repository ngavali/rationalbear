import sqlite3

DB_NAME = "tasks.db"

def get_connection():
    return sqlite3.connect(DB_NAME)

def init_db():
    conn = get_connection()
    cursor = conn.cursor()

    print("Initializing DB...")

    cursor.execute("""
    CREATE TABLE IF NOT EXISTS tasks (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        description TEXT,
        type TEXT,
        status TEXT,
        priority TEXT,
        energy TEXT,
        created_at TEXT,
        due_date TEXT,
        completed_at TEXT,
        tags TEXT,
        metadata TEXT
    )
    """)

    cursor.execute("""
    CREATE TABLE IF NOT EXISTS time_sessions (
        id TEXT PRIMARY KEY,
        task_id TEXT,
        start_time TEXT,
        end_time TEXT,
        duration_seconds INTEGER,
        type TEXT,
        FOREIGN KEY(task_id) REFERENCES tasks(id)
    )
    """)

    cursor.execute("""
    CREATE TABLE IF NOT EXISTS task_notes (
    id TEXT PRIMARY KEY,
    task_id TEXT,
    note TEXT,
    created_at TEXT,
    FOREIGN KEY(task_id) REFERENCES tasks(id)
    )
    """)

    conn.commit()
    conn.close()
