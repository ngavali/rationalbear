import uuid
import json
from datetime import datetime
from db import get_connection

import uuid
from datetime import datetime
from db import get_connection

def add_note(task_id, note):
    conn = get_connection()
    cursor = conn.cursor()

    cursor.execute("""
    INSERT INTO task_notes VALUES (?, ?, ?, ?)
    """, (
        str(uuid.uuid4()),
        task_id,
        note,
        datetime.utcnow().isoformat()
    ))

    conn.commit()
    conn.close()

def get_notes(task_id):
    conn = get_connection()
    cursor = conn.cursor()

    cursor.execute("""
    SELECT note, created_at
    FROM task_notes
    WHERE task_id = ?
    ORDER BY created_at DESC
    """, (task_id,))

    rows = cursor.fetchall()
    conn.close()

    return rows

def get_pending_tasks():
    conn = get_connection()
    cursor = conn.cursor()

    cursor.execute("SELECT id, title, type, priority FROM tasks WHERE status != 'done'")
    rows = cursor.fetchall()

    conn.close()
    return rows

def create_task(title, description="", task_type="personal",
                priority="medium", energy="medium", due_date=None, tags=None):

    conn = get_connection()
    cursor = conn.cursor()

    task_id = str(uuid.uuid4())
    now = datetime.utcnow().isoformat()

    cursor.execute("""
INSERT INTO tasks (
    id, title, description, type, status,
    priority, energy, created_at,
    due_date, completed_at, tags, metadata
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
""", (
    task_id,
    title,
    description,
    task_type,
    "pending",
    priority,
    energy,
    now,
    due_date,
    None,
    json.dumps(tags or []),
    json.dumps({})
))

    conn.commit()
    conn.close()

    return task_id


def get_all_tasks():
    conn = get_connection()
    cursor = conn.cursor()

    cursor.execute("SELECT * FROM tasks")
    rows = cursor.fetchall()

    conn.close()
    return rows


def mark_task_done(task_id):
    conn = get_connection()
    cursor = conn.cursor()

    cursor.execute("""
    UPDATE tasks
    SET status = ?, completed_at = ?
    WHERE id = ?
    """, ("done", datetime.utcnow().isoformat(), task_id))

    conn.commit()
    conn.close()


def delete_task(task_id):
    conn = get_connection()
    cursor = conn.cursor()

    cursor.execute("DELETE FROM tasks WHERE id = ?", (task_id,))
    conn.commit()
    conn.close()

def get_time_summary(start, end):
    conn = get_connection()
    cursor = conn.cursor()

    cursor.execute("""
SELECT t.title, SUM(ts.duration_seconds)
FROM time_sessions ts
JOIN tasks t ON ts.task_id = t.id
WHERE ts.start_time BETWEEN ? AND ?
GROUP BY ts.task_id, t.title
ORDER BY SUM(ts.duration_seconds) DESC
    """, (start.isoformat(), end.isoformat()))

    rows = cursor.fetchall()
    conn.close()

    return rows
