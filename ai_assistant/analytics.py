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

from datetime import datetime, timedelta

def get_date_range(period):
    now = datetime.now()

    if period == "Day":
        start = now.replace(hour=0, minute=0, second=0, microsecond=0)

    elif period == "Week":
        start = now - timedelta(days=now.weekday())  # Monday start
        start = start.replace(hour=0, minute=0, second=0, microsecond=0)

    elif period == "Month":
        start = now.replace(day=1, hour=0, minute=0, second=0, microsecond=0)

    elif period == "Year":
        start = now.replace(month=1, day=1, hour=0, minute=0, second=0, microsecond=0)

    return start, now
