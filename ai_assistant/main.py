#!myenv/bin/python3

import questionary
from db import init_db
from datetime import datetime
from analytics import get_today_summary
from timer import active_timer, start_timer, stop_timer, start_countdown, select_duration
from task_manager import create_task, get_all_tasks, mark_task_done, delete_task, get_pending_tasks, add_note, get_notes

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
            value=(t[0], t[1])
        )
        for t in tasks
    ]

    result = questionary.select(
        action,
        choices=choices
    ).ask()

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
        print("""
1. Add Task
2. View Tasks
3. Complete Task
4. Delete Task
5. Start Timer
6. Stop Timer
7. Countdown Timer
8. Today's Summary
9. Add Note to Task
10. View Task Notes
11. Exit
""")

        choice = input("Choose: ")

        if choice == "1":
            title = input("Title: ")

            task_type = questionary.select(
                "Select Type:",
                choices=["learning", "personal", "deep_work"]
            ).ask()

            priority = questionary.select(
                "Select Priority:",
                choices=["low", "medium", "high"],
                default="medium"
            ).ask()

            energy = questionary.select(
                "Select Energy:",
                choices=["low", "medium", "high"]
            ).ask()

            create_task(title, task_type=task_type,
                        priority=priority, energy=energy)

        elif choice == "2":
            tasks = get_all_tasks()
            print_tasks(tasks)

        elif choice == "3":
            task_id = select_task(filter_status="pending", action="Select task to complete")
            if not task_id:
                continue
            mark_task_done(task_id)

        elif choice == "4":
            task_id = select_task(action="Select task to delete")
            if not task_id:
                continue
            confirm = questionary.confirm("Are you sure?").ask()
            if confirm:
                delete_task(task_id)

        elif choice == "5":
            task_id, task_title = select_task(filter_status="pending", action="Select task to start timer")
            if not task_id:
                continue

            start_timer(task_id, task_title)

        elif choice == "6":
            print(f"Stopping timer for task: {task_id}")
            stop_timer()

        elif choice == "7":
            task_id, task_title = select_task(filter_status="pending", action="Select task for countdown")
            if not task_id:
                continue
            minutes = int(select_duration())
            start_countdown(task_id, task_title, minutes)

        elif choice == "8":
            get_today_summary()

        elif choice == "9":
            task_id, task_title = select_task(action="Select task to add note")

            if task_id:
                note = input("Enter note: ")
                add_note(task_id, note)
                print("✅ Note added")

        elif choice == "10":
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

        elif choice == "11":
            break

if __name__ == "__main__":
    main()
