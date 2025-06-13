import { readDir, BaseDirectory, readTextFile } from "@tauri-apps/plugin-fs"
import { marked } from "marked";
import DOMPurify from "dompurify";
import {listen} from "@tauri-apps/api/event";
import { Window } from "@tauri-apps/api/window";

const tabNavigation = document.querySelector('.tab-navigation');
const tabContent = document.querySelector('.tab-content');
const datasetWindow = new Window('dataset_mng');

const theme = localStorage.getItem('color-theme');
document.documentElement.setAttribute('color-theme', theme || 'light');

listen('theme-changed', (event) => {
    document.documentElement.setAttribute('color-theme', event.payload);
})

marked.setOptions({
    breaks: true, // <- enables line breaks on single newlines
    gfm: true,
});

async function createTab(dataset) {
    // Create tab button
    const tabButton = document.createElement('button');
    tabButton.className = 'tab-button';
    tabButton.textContent = dataset;

    let help_content = await readTextFile(`dataset/${dataset}.md`, { baseDir: BaseDirectory.AppData })

    // Create tab content
    const content = document.createElement('div');
    content.className = 'dataset-content';
    content.innerHTML = `
        <div class="dataset-header">
            <h1>${dataset}</h1>
        </div>
        <hr/>
        <div class="dataset-help">
            ${DOMPurify.sanitize(marked.parse(help_content))}
        </div>
    `;

    // Add click handler
    tabButton.addEventListener('click', () => {
        // Remove active class from all tabs and contents
        document.querySelectorAll('.tab-button').forEach(btn => btn.classList.remove('active'));
        document.querySelectorAll('.dataset-content').forEach(content => content.classList.remove('active'));

        // Add active class to clicked tab and its content
        tabButton.classList.add('active');
        content.classList.add('active');
    });

    return { tabButton, content };
}

async function load_datasets() {
    let datasets = (await readDir("dataset", { baseDir: BaseDirectory.AppData }))
        .filter(entry => entry.name?.endsWith('.csv') && !entry.children)
        .map(entry => entry.name.slice(0, -4));
    console.log("Datasets found:", datasets);

    tabNavigation.innerHTML = '';
    tabContent.innerHTML = '';

    if (datasets.length === 0) {
        tabContent.innerHTML = '<p class="empty-state">No datasets loaded</p>';
    }
    else {
        for (const dataset of datasets) {
            const { tabButton, content } = await createTab(dataset);
            tabNavigation.appendChild(tabButton);
            tabContent.appendChild(content);
        };
        tabContent.children[0].classList.add('active'); // Activate the first tab by default
        tabNavigation.children[0].classList.add('active'); // Activate the first tab button by default
    }
}

window.addEventListener('keydown', (e) => {
  if (e.key === "Escape") {
    e.preventDefault();
    // Use either close or hide
    // await appWindow.close(); // Quits the window
    datasetWindow.hide(); //hide the window
    return;
  }}
)

load_datasets()