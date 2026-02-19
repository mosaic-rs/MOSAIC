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


const { open } = window.__TAURI__.dialog;
const { save } = window.__TAURI__.dialog;
const { mkdir } = window.__TAURI__.fs;

const createProjectFolder = async () => {
    try {
        const projectPath = await save({
            title: 'Create New Project',
            defaultPath: 'New Project'
        });

        if (projectPath) {
            await mkdir(projectPath, { recursive: true });
            
            console.log("Folder created at:", projectPath);
        }
    } catch (e) {
        console.error("Failed to create folder:", e);
    }
};

const openFinder = async () => {
    try {
        const selected = await open({
            multiple: false,
            directory: true,
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