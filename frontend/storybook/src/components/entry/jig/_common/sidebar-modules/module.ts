import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/jig/_common/sidebar-modules/module";
import "@elements/entry/jig/edit/sidebar/module/window";
import "@elements/entry/jig/edit/sidebar/module/menu";
import "@elements/core/buttons/icon";
import "@elements/core/menu/kebab";
import "@elements/core/menu/menu-line";
import {ModuleKind, moduleKinds} from "@elements/module/_common/types";

const STR_CUSTOM_COPY = "Copy to another Jig";

export default {
    title: "Entry / Jig / Sidebar modules"
}

interface Args {
    state: "draft" | "complete",
    thumbnail: string,
    selected: boolean,
    module: ModuleKind | "none",
    rawIndex: number,
    menuOpen: boolean,
    showAdvancedMenu: boolean,
    makeDemoRoomAtTop: boolean,
    showAdd: boolean,
    dragging: boolean,
    collapsed: boolean,
    dragX: number,
    dragY: number,
}

const DEFAULT_ARGS:Args = {
    state: "draft",
    module: "cover",
    thumbnail : "",
    selected: true,
    rawIndex: 0,
    menuOpen: false,
    showAdvancedMenu: false,
    makeDemoRoomAtTop: true,
    showAdd: true,
    dragging: false,
    collapsed: false,
    dragX: 0,
    dragY: 0,
}

type InternalExtra = {
    slot?: string,
}

export const Module = (props?:Partial<Args> & InternalExtra) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;
    
    const {state, slot, dragX, dragY, showAdd, thumbnail, showAdvancedMenu, rawIndex, makeDemoRoomAtTop, menuOpen, ...rest} = props;
    const moduleProps:any = rest;

    const windowProps = {
        state: moduleProps.module === "none" ? "empty" : state, 
        thumbnail
    };

    moduleProps.index = rawIndex;
    if(moduleProps.module === "none") {
        moduleProps.module = "";
    }

    const style = moduleProps.dragging
        ? `position: fixed; top: 0; left: 0; z-index: 9999; transform: translate(${dragX}px, ${dragY}px);`
        : "";

    return `
        <div style="${makeDemoRoomAtTop && `position: absolute; top: 200px;`}" ${slot && `slot="${slot}"`}>
            <jig-sidebar-module style="${style}" ${argsToAttrs(moduleProps)}>
                <jig-edit-sidebar-module-window ${argsToAttrs(windowProps)} slot="window"></jig-edit-sidebar-module-window>
                ${renderMenu(menuOpen, showAdvancedMenu)} 
                ${showAdd && `<button-icon icon="gears" slot="add"></button-icon>`}
            </jig-sidebar-module>
        </div>`;
}

function renderMenu(visible: boolean, showAdvanced:boolean) {
    return `
        <menu-kebab ${visible && "visible"} slot="menu">
            <jig-edit-sidebar-module-menu ${showAdvanced && "advanced"}>
            <menu-line slot="lines" icon="edit"></menu-line>
            <menu-line slot="lines" icon="move-up"></menu-line>
            <menu-line slot="lines" icon="move-down"></menu-line>
            <menu-line slot="lines" icon="duplicate"></menu-line>
            <menu-line slot="lines" icon="delete"></menu-line>
            <menu-line slot="advanced" icon="copy" customLabel="${STR_CUSTOM_COPY}"></menu-line>
            </jig-edit-sidebar-module-menu>
        </menu-kebab>
    `;
    //
}

Module.args = DEFAULT_ARGS;

Module.argTypes = {
    module: {
        control: {
            type: 'inline-radio',
            options: ["none"].concat(moduleKinds)
        }
    },
    state: {
        control: {
            type: 'inline-radio',
            options: ["draft", "complete"] 
        }
    }
}
