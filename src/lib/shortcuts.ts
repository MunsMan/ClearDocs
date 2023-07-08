export const keyboard_shortcuts = (event: KeyboardEvent, div: HTMLDivElement) => {
    console.log(event);
    let key = event.key.toUpperCase();
    if (event.ctrlKey) {
        key = 'C-' + key;
    }
    shortcut_register[key](div);
};

const halfPageDown = (scroll_div: HTMLDivElement) => {
    console.log(scroll_div.clientHeight / 2);
    scroll_div.scrollBy(0, scroll_div.clientHeight / 2);
};
const halfPageUp = (scroll_div: HTMLDivElement) => {
    console.log(scroll_div.clientHeight / 2);
    scroll_div.scrollBy(0, -scroll_div.clientHeight / 2);
};

const shortcut_register: { [key: string]: (scroll_div: HTMLDivElement) => void } = {
    'C-D': halfPageDown,
    'C-U': halfPageUp
};
