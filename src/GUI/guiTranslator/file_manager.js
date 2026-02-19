/*
This file is part of MOSAIC.

MOSAIC is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

MOSAIC is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
MOSAIC. If not, see <https://www.gnu.org/licenses/>.
*/

//import { openPath, openUrl } from '@tauri-apps/plugin-opener';


//await openPath('/path/to/file');
//await openPath('C:/path/to/file', 'vlc');
//await openUrl('https://tauri.app');

const { open } = window.__TAURI__.dialog;
const { save } = window.__TAURI__.dialog;
const { mkdir } = window.__TAURI__.fs;

const createProjectFolder = async () => {
    try {
        // 1. Open the "Save As" window to let the user name the project
        const projectPath = await save({
            title: 'Create New Project',
            defaultPath: 'New Project'
        });

        if (projectPath) {
            // 2. Create the directory at that exact path
            // Setting 'recursive: true' ensures parent folders are created if needed
            await mkdir(projectPath, { recursive: true });
            
            console.log("Success! Folder created at:", projectPath);
            alert("Project folder created!");
        }
    } catch (err) {
        console.error("Failed to create folder:", err);
    }
};

const openFinder = async () => {
    try {
        const selected = await open({
            multiple: false,
            directory: false,
        });
        
        if (selected) {
            console.log("File selected:", selected);
        }
    } catch (e) {
        console.error("Dialog error:", e);
    }
};

document.addEventListener('DOMContentLoaded', () => {
    const createProjectButton = document.getElementById('createProjectButton');
    const openProjectButton = document.getElementById('openProjectButton')
    
    if (createProjectButton) {
        createProjectButton.onclick = createProjectFolder;
    } if (openProjectButton) {
        openProjectButton.onclick = openFinder;
    }else {
        console.error("Could not find button with ID");
    }
});