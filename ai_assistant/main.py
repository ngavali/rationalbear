#!myenv/bin/python3

import questionary
from db import init_db
from datetime import datetime
from questionary import Style
from analytics import get_today_summary, get_date_range
from timer import active_timer, start_timer, stop_timer, start_countdown, select_duration
from task_manager import create_task, get_all_tasks, mark_task_done, delete_task, get_pending_tasks, add_note, get_notes, get_time_summary

import questionary
from questionary import Style

custom_style = Style([
    ('qmark', 'fg:#8ecae6'),
    ('question', 'fg:#e0fbfc bold'),
    ('answer', 'fg:#90dbf4 bold'),
    ('pointer', 'fg:#ffb703 bold'),
    ('selected', 'fg:#fefeff bg:#1d1e1e'),
    ('highlighted', 'fg:#a8dadc'),
])

def select(prompt, choices, default=None,allow_back=True):
    if allow_back:
        choices = ["< Back"] + choices

    try:
        answer = questionary.select(
            prompt,
            choices=choices,
            default=default,
            style=custom_style,
        ).ask()

        if answer == "< Back":
            return None

        return answer

    except KeyboardInterrupt:
        return None

def confirm(prompt):
    return questionary.confirm(
        prompt,
        style=custom_style
    ).ask()


def text(prompt):
    return questionary.text(
        prompt,
        style=custom_style
    ).ask()

def select_summary_period():
    return select("Select Summary Range:", [
        "Day",
        "Week",
        "Month",
        "Year"
    ])

def show_summary():
    period = select_summary_period()

    if period == None:
        return

    start, end = get_date_range(period)
    data = get_time_summary(start, end)

    print(f"\n📊 {period} Summary:\n")

    total = 0

    for title, seconds in data:
        mins = seconds // 60
        print(f"{title} → {mins} mins")
        total += seconds

    print(f"\nTotal: {total // 60} mins\n")

def show_menu():
    return select(
        "What do you want to do?",
        choices=[
            "Add Task",
            "View Tasks",
            "Complete Task",
            "Delete Task",
            "Start Timer",
            "Stop Timer",
            "Countdown Timer",
            "Summary",
            "Add Note",
            "View Notes",
            "Exit"
        ],
        allow_back=False
    )

def select_task(filter_status=None, action="Select Task"):
    tasks = get_all_tasks()

    if filter_status:
        tasks = [t for t in tasks if t[4] == filter_status]

    if not tasks:
        print("No tasks available")
        return None,None

    choices = [
        questionary.Choice(
            title=f"{t[1]} ({t[3]} | {t[4]})",
            value=(t[0], t[1]),
        )
        for t in tasks
    ]

    result = select(
        action,
        choices=choices
    )

    if result == None:
        return (None, None)

    return result

def print_tasks(tasks):
    for t in tasks:
        print(f"""
ID: {t[0]}
Title: {t[1]}
Type: {t[3]} | Status: {t[4]}
Priority: {t[5]} | Energy: {t[6]}
Due: {t[8]}
------------------------
""")

def main():
    init_db()

    while True:
        choice = show_menu()

        if choice == "Add Task":
            title = input("Title: ")

            task_type = select(
                "Select Type:",
                choices=["learning", "personal", "deep_work"]
            )

            priority = select(
                "Select Priority:",
                choices=["low", "medium", "high"],
                default="medium"
            )

            energy = select(
                "Select Energy:",
                choices=["low", "medium", "high"]
            )

            create_task(title, task_type=task_type,
                        priority=priority, energy=energy)

        elif choice == "View Tasks":
            tasks = get_all_tasks()
            print_tasks(tasks)

        elif choice == "Complete Task":
            task_id = select_task(filter_status="pending", action="Select task to complete")
            if not task_id:
                continue
            mark_task_done(task_id)

        elif choice == "Delete Task":
            task_id = select_task(action="Select task to delete")
            if not task_id:
                continue
            confirm = confirm("Are you sure?")
            if confirm:
                delete_task(task_id)

        elif choice == "Start Timer":
            task_id, task_title = select_task(filter_status="pending", action="Select task to start timer")
            if not task_id:
                continue

            start_timer(task_id, task_title)

        elif choice == "Stop Timer":
            print(f"Stopping timer for task: {task_id}")
            stop_timer()

        elif choice == "Countdown Timer":
            task_id, task_title = select_task(filter_status="pending", action="Select task for countdown")
            if not task_id:
                continue
            minutes = int(select_duration())
            start_countdown(task_id, task_title, minutes)

        elif choice == "Summary":
            show_summary()

        elif choice == "Add Note":
            task_id, task_title = select_task(action="Select task to add note")

            if task_id:
                note = input("Enter note: ")
                add_note(task_id, note)
                print("✅ Note added")

        elif choice == "View Notes":
            task_id, task_title = select_task(action="Select task to view notes")

            if task_id:
                notes = get_notes(task_id)
                print(f"\n📝 Notes for: {task_title}\n")
                for note, ts in notes:
                    dt = datetime.fromisoformat(ts)

                    formatted = dt.strftime("%d %b %Y %H:%M")

                    print(f"[{formatted}]")
                    print(f"- {note}\n")

                print(50*"-")

        elif choice == "Exit":
            break

if __name__ == "__main__":
    main()
