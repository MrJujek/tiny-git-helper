#!/usr/bin/env node

import showSettingsMenu, { loadSavedSettings } from "./settings.js";

import showCloneMenu from "./clone.js";
import showHelp from "./help.js";

import chalk from "chalk";
import inquirer from "inquirer";

const settings = {
  username: "",
  sorting: "",
  protocol: "",
};

await loadSavedSettings(settings);
await parseArgs();

async function showMenu(): Promise<void> {
  console.log("Welcome to git-helper! \n");
  const answers = await inquirer.prompt({
    name: "menu_action",
    type: "list",
    message: "What can I do for you?",
    choices: ["Clone repo", "Edit settings", "Help", "Exit"],
  });

  return handleMenuChoice(answers.menu_action);
}

async function handleMenuChoice(choice: string) {
  switch (choice) {
    case "Clone repo":
      await showCloneMenu(settings);
      return showMenu();
    case "Edit settings":
      await showSettingsMenu(settings);
      return showMenu();
    default:
      return process.exit(0);
  }
}

async function parseArgs() {
  if (process.argv.slice(2).length === 0) {
    return showMenu();
  } else {
    switch (process.argv.slice(2)[0]) {
      case "clone":
        return showCloneMenu(settings);
      case "settings":
        return showSettingsMenu(settings);
      case "help":
        return showHelp();
    }
  }
}
