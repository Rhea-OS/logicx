import * as obs from 'obsidian';

import LogicxView from "./view.js";
import SettingsTab from "./settings.js";

import * as logicx from './wasm.js';

export const LOGICX_VIEW = "logicx-view";

export interface Settings {

}

export const default_settings: Settings = {

};

export default class Logicx extends obs.Plugin {

    settingsTab: SettingsTab | null = null;
    settings: Settings = default_settings;

    currentFile: obs.TFile | null = null;

    async onload() {
        this.registerView(LOGICX_VIEW, leaf => new LogicxView(leaf, this));
        this.registerExtensions(["logicx", "logic"], LOGICX_VIEW);

        const self = this;
        this.registerEvent(this.app.workspace.on('file-open', file => self.currentFile = file));

        this.addCommand({
            id: "new-logicx-component",
            name: "New LogicX Component",
            async callback() {
                // TODO: Handle existing files
                const newFile = `${obs.normalizePath(self.currentFile?.parent?.path ?? self.app.vault.getRoot()?.path)}/component.logicx`;

                let ctx = new logicx.LogicXContext();

                const tfile = await self.app.vault.create(newFile, ctx.getData());
                await self.app.workspace.getLeaf(false).openFile(tfile);
            }
        })

        this.addSettingTab(new SettingsTab(this.app, this));
    }
}