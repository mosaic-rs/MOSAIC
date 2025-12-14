"""
This verifies that projects, their participants, and trials exist when executing commands
This is used so we can a) handle errors better, and b) verifiy before we update the global session which will cause
TONS of errors
"""

import os

class SystemVerifier:
    def __init__(self):
        pass

    @staticmethod
    def project_verifier(path: str)-> None:
        path = os.path.abspath(path)
        if not os.path.exists(path):
            raise RuntimeError(f"Error: ~{path} does not exist or can not be found.")

    @staticmethod
    def participant_dir_verifier(path: str)-> None:
        path = os.path.abspath(path+"/participants")
        if not os.path.exists(path):
            raise RuntimeError(f"Error: ~{path} does not exist or can not be found.")

    @staticmethod
    def participant_verifier(path: str, ID)-> None:
        path = os.path.join(path, "participants", ID)
        path = os.path.abspath(path)
        if not os.path.exists(path):
            raise RuntimeError(f"Error: ~{path} does not exist or can not be found.")

    @staticmethod
    def trial_verifier(path: str, ID: str=None)-> None:
        if ID is None:
            path = os.path.join(path)
            if not os.path.exists(path):
                raise RuntimeError(f"Error: ~{path} does not exist or can not be found.")

        else:
            path = os.path.join(path, ID)
            if not os.path.exists(path):
                raise RuntimeError(f"Error: ~{path} does not exist or can not be found.")