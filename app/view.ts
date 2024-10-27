import * as obs from 'obsidian';
import {LogicxFile} from "./LogicxFile.js";

import Logicx, {LOGICX_VIEW} from "./main.js";
import * as wasm from './wasm.js';

export default class LogicxView extends obs.TextFileView implements LogicxFile {
    constructor(leaf: obs.WorkspaceLeaf, private plugin: Logicx) {
        super(leaf);
    }

    getViewData(): string {

    }

    setViewData(data: string, clear: boolean): void {

    }

    clear(): void {

    }

    getViewType(): string {
        return LOGICX_VIEW;
    }

    onload() {
        wasm.mount(this.contentEl);
    }
}