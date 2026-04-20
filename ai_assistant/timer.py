import time
import uuid
import threading
import questionary
from datetime import datetime
from db import get_connection
from playsound import playsound

active_timer = None

def play_alarm():
    try:
        playsound("./assets/alarm.mp3")  # place file in same folder
    except Exception as e:
        print("⚠️ Could not play sound:", e)

def play_alarm_async():
    threading.Thread(target=play_alarm).start()

def start_timer(task_id, task_title):
    global active_timer

    if active_timer:
        print("⚠️ A timer is already running")
        return

    active_timer = {
        "task_id": task_id,
        "title": task_title,
        "start_time": time.time()
    }

    print(f"⏱️ Timer started for: {task_title}")

def stop_timer():
    global active_timer

    if not active_timer:
        print("No active timer")
        return

    end_time = time.time()
    duration = int(end_time - active_timer["start_time"])

    save_session(
        active_timer["task_id"],
        active_timer["start_time"],
        end_time,
        duration,
        "stopwatch"
    )

    active_timer = None
    print(f"✅ Session recorded: {duration} seconds")

def select_duration():
    choice = questionary.select(
        "Select Duration:",
        choices=[
            "10 min",
            "15 min",
            "30 min",
            "60 min",
            "120 min",
            "Custom"
        ]
    ).ask()

    if choice == "Custom":
        minutes = int(input("Enter minutes: "))
    else:
        minutes = int(choice.split()[0])

    return minutes

def start_countdown(task_id, task_title, minutes):
    total_seconds = minutes * 60
    remaining = total_seconds

    print(f"⏳ Countdown started for {task_title} ({minutes} mins)")
    print("Press Ctrl + C to stop early\n")

    start_time = time.time()
    audio_triggered = False

    try:
        while remaining > 0:
            mins, secs = divmod(remaining, 60)
            print(f"{mins:02d}:{secs:02d}", end="\r")

            # 🔊 Trigger at last 21 seconds
            if remaining == 21 and not audio_triggered:
                play_alarm_async()
                audio_triggered = True

            time.sleep(1)
            remaining -= 1

        print("\n⏰ Time's up!")
        confirm = True

    except KeyboardInterrupt:
        print("\n⛔ Countdown stopped early!")
        confirm = questionary.confirm("Save this session?").ask()

    end_time = time.time()
    duration = int(end_time - start_time)

    if confirm:
        save_session(task_id, start_time, end_time, duration, "countdown")
        print(f"✅ Recorded: {duration // 60} min {duration % 60} sec")


def save_session(task_id, start, end, duration, session_type):
    conn = get_connection()
    cursor = conn.cursor()

    cursor.execute("""
    INSERT INTO time_sessions VALUES (?, ?, ?, ?, ?, ?)
    """, (
        str(uuid.uuid4()),
        task_id,
        datetime.fromtimestamp(start).isoformat(),
        datetime.fromtimestamp(end).isoformat(),
        duration,
        session_type
    ))

    conn.commit()
    conn.close()
