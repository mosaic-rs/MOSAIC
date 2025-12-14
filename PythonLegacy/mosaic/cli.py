from mosaic.shell import OASShell
from mosaic.Session.session_manager import SessionManagement

def main():
    session = SessionManagement()
    shell = OASShell(session)
    shell.start()

# THIS WILL ALL BE DOCSTRINGED AS



""""
def main():
    parser = argparse.ArgumentParser(
        prog="Oromotor Asymmetry System (OAS) CLI",
        description="OAS Calculates mouth movement and compares that to a praat analysis."
    )
    subparsers = parser.add_subparsers(dest="target")

    # SHELL CMD

    shell_parser = subparsers.add_parser("shell", help="Start the OAS interactive shell")

    # GLOBAL/SESSION LEVEL COMMANDS

    session_parser = subparsers.add_parser("session", help="Session commands")
    session_sub = session_parser.add_subparsers(dest="action")

    session_reset = session_sub.add_parser("reset", help="Reset OAS session state")
    session_summary = session_sub.add_parser("c", help="See what project, participant, or trial directory you are in")

    # PROJECT LEVEL COMMANDS/ARGUMANTS

    project_parser = subparsers.add_parser("project", help="Project level commands")
    project_sub = project_parser.add_subparsers(dest="action")


    project_create = project_sub.add_parser("create", help="Create a new OAS project")
    project_create.add_argument("path")
    project_create.add_argument("name")


    project_delete = project_sub.add_parser("delete", help="Delete an OAS project")
    project_delete.add_argument("path")

    project_open = project_sub.add_parser("open", help="Open an OAS project")
    project_open.add_argument("path", help="Path to the project")


    # PARTICIPANT LEVEL ARGUMENTS

    participant_parser = subparsers.add_parser("participant", help="Participant level commands")
    participant_sub = participant_parser.add_subparsers(dest="action")

    participant_open = participant_sub.add_parser("open", help="Open a participant file")
    participant_open.add_argument("ID")

    participant_create = participant_sub.add_parser("create", help="Create a participant file")
    participant_create.add_argument("ID")

    participant_list = participant_sub.add_parser("list", help="View the participants in your project")

    participant_delete = participant_sub.add_parser("delete", help="Delete participant file")
    participant_delete.add_argument("ID")

    participant_view = participant_sub.add_parser("view", help="View participant information")


    # TRIAL LEVEL ARGUMENTS

    trial_parser = subparsers.add_parser("trial", help="Trial level commands")
    trial_sub = trial_parser.add_subparsers(dest="action")

    trial_open = trial_sub.add_parser("open", help="Open a trial")
    trial_open.add_argument("ID")


    args = parser.parse_args()

    session = SessionManagement()
    print(session.get_prompt())

    if args.target == "shell":
        session = SessionManagement()
        shell = OASShell(session)
        shell.start()
        return

    if args.target == "session":
        if args.action == "reset":
            session = SessionManagement()
            session.clear_session()
            print("Session reset. No project, participant, or trial is currently active.")
            return
        if args.action == "c":
            session = SessionManagement()
            print(session.session_summary())


    if args.target == "project":
        if args.action == "create":

            print(f"project path: {args.path}\nProject Name: {args.name}")
            from mosaic.SystemManager.project.project_creation import ProjectCreation
            ProjectCreation().project_creation(args.path, args.name)

            # Nowe we put them in that project

            session = SessionManagement()
            session.set_current_project(os.path.join(args.path, args.name))

        elif args.action == "delete":
            print(f"Deleting ~/{args.path}\nWill add actual logic later")

        elif args.action == "open":
            print(f"Opening ~/{args.path}...")
            # first we gotta verify it exists:
            try:
                SystemVerifier.project_verifier(args.path)

            except RuntimeError as e:
                print(f"[OAS ERROR] {e}")
                return

            # when we know it exists, we then update the sessions data

            session = SessionManagement(args.path)
            session.set_current_project(args.path)
            print(f"Opened project at: {args.path}")
            print(session.session_summary())


    # PARTICIPANT LEVEL COMMANDS/ARGUMENTS

    try:
        if args.target == "participant":
            if args.action == "open":
                # first we have to verify they are in a project
                session = SessionManagement()
                session.require_project()

                # then we verify the participant folder exists
                session_data = session._read_session()

                try:
                    SystemVerifier.participant_verifier(session_data["currentProject"], args.ID)

                except RuntimeError as e:
                    print(f"[OAS ERROR] {e}")
                    print(session.session_summary())

                # Once that is done we edit the Session manager to write the participant we are in
                session.set_current_participant(str(args.ID))

                print(f"Participant file opened at at: {session_data['currentProject']}/participants/{args.ID}")
                print(session.session_summary())

            if args.action == "create":
                # first we have to verify they are in a project
                session = SessionManagement()
                session.require_project()

                # we need to get proj directory so we can make the participant folder
                session_data = session._read_session()
                directory = session_data["currentProject"]
                # now we create the participant folder
                from mosaic.SystemManager.participants.participant_creation import ParticipantCreation
                try:
                    ParticipantCreation().creating_participant_directory(directory, args.ID)

                except RuntimeError as e:
                    print(f"[OAS ERROR] {e}")
                    return

            if args.action == "list":
                # first we have to verify they are in a project
                session = SessionManagement()
                session.require_project()

                # get project path so we can get the list of participants
                session_data = session._read_session()
                directory = session_data["currentProject"]

                # check that the participants subdir exists
                SystemVerifier.participant_dir_verifier(directory)

                # now we can get the list of participants inside

                participants = ParticipantList.get_participants(directory)
                print(f"Participants: {participants}")

            if args.action == "view":
                # check they are in a project
                session = SessionManagement()
                session.require_project()

                # check they are in a participant folder

                session.require_participant()

                # we want to get the participant file/path from the session data
                path = session._read_session().get("currentParticipant")

                # Now we can access the data in the participant folder the user is "logged in" to
                try:
                    ParticipantViewer.participant_viewer(path)

                except Exception as e:
                    print(f"[OAS ERROR] {e}")


        # TRIAL LEVEL COMMANDS/ARGS

    except RuntimeError as e:
        print(f"[OAS ERROR] {e}")
        return



    # TRIAL LEVEL COMMANDS/ARGUMENTS

    try:
        if args.target == "trial":
            if args.action == "open":
                pass

    except RuntimeError as e:
        print(f"[OAS ERROR] {e}")
        return

"""
"""

HOW TO USE CLI:

When running locally from command terminal, use:

> source .venv/bin/activate

Then use: 

> python -m mosaic.cli -m (mode) -f (input file) -o (output file) -f (force) -start (compute at row int) -end (finish at row int)

CLI testing command

python3 -m mosaic.cli -m init -f "data/test-data/v15044gf0000d1dlc67og65r2deqmhd0.csv" --force --pose

"""

if __name__ == "__main__":
    main()