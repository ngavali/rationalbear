from db import get_connection
from datetime import datetime

def get_today_summary():
    conn = get_connection()
    cursor = conn.cursor()

    today = datetime.utcnow().date().isoformat()

    cursor.execute("""
    SELECT t.title, SUM(ts.duration_seconds)
    FROM time_sessions ts
    JOIN tasks t ON ts.task_id = t.id
    WHERE date(ts.start_time) = ?
    GROUP BY ts.task_id
    """, (today,))

    rows = cursor.fetchall()
    conn.close()

    print("\n📊 Today's Time Summary:\n")

    total = 0

    for title, seconds in rows:
        mins = seconds // 60
        print(f"{title} → {mins} mins")
        total += seconds

    print(f"\nTotal: {total // 60} mins\n")
