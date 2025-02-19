import * as obs from 'obsidian';
import SettingsTab from "./settings.js";
import LogicxView from "./view.js";

import mod from 'logicx/logicx_bg.wasm';

import * as logicx from 'logicx/logicx_bg.js';

export {
    logicx,
};

export const LOGICX_VIEW = "logicx-view";

export interface Settings {

}

export const default_settings: Settings = {

};

export default class LogicX extends obs.Plugin {
    settingsTab: SettingsTab | null = null;
    settings: Settings = default_settings;

    currentFile: obs.TFile | null = null;

    constructor(app: obs.App, manifest: obs.PluginManifest) {
        super(app, manifest);

        loadWasm()
            .finally();
    }

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

async function loadWasm() {
    const wasm = await WebAssembly.compile(mod)
        .then(mod => WebAssembly.instantiate(mod, {
            "./logicx_bg.js": logicx
        }));

    logicx.__wbg_set_wasm(wasm.exports);
}