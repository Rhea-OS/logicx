import * as obs from 'obsidian';
import {LogicxFile} from "./LogicxFile.js";

import Logicx, {LOGICX_VIEW} from "./main.js";
import * as wasm from './wasm.js';

export default class LogicxView extends obs.TextFileView implements LogicxFile {

    private logicx: wasm.LogicXContext;
    private toggleEdit: obs.ExtraButtonComponent;

    constructor(leaf: obs.WorkspaceLeaf, private plugin: Logicx) {
        super(leaf);

        this.logicx = new wasm.LogicXContext();
        this.toggleEdit = new obs.ExtraButtonComponent(this.containerEl.querySelector(".view-actions")!)
            .setIcon(this.logicx.getState().edit ? 'pencil' : 'play')
            .onClick(() => {
                const state = this.logicx.getState();
                return this.logicx.setState(state.withEdit(!state.edit));
            });
    }

    getViewData(): string {
        return this.logicx.getData();
    }

    setViewData(data: string, clear: boolean): void {
        this.logicx.setData(data, clear);
    }

    clear(): void {
        this.logicx.clear();
    }

    getViewType(): string {
        return LOGICX_VIEW;
    }

    onload() {
        if (this.contentEl instanceof HTMLDivElement)
            this.logicx.mount(this.contentEl);
        else
            this.logicx.mount(this.contentEl.createDiv());

        this.logicx.onStateChanged(state => this.toggleEdit.setIcon(state.edit ? 'pencil' : 'play'));
    }
}