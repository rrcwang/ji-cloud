
export const moduleKinds:Array<ModuleKind> = [ 
    "cover",
    "flashcards",
    "matching",
    "memory",
    "poster",
    "tapping-board",
    "tracing",
    "video",
    "card-quiz",
    "drag-drop",
];
export type ModuleKind =
    "cover"
    | "flashcards"
    | "matching"
    | "memory"
    | "poster"
    | "tapping-board"
    | "tracing"
    | "video"
    | "card-quiz"
    | "drag-drop";

export const GET_STR_MODULE = (kind: ModuleKind) => {
    switch(kind) {
        case "cover": return "Cover";
        case "flashcards": return "Flashcards";
        case "matching": return "Matching";
        case "memory": return "Memory Game";
        case "poster": return "Poster";
        case "tapping-board": return "Tapping Board";
        case "tracing": return "Tracing";
        case "video": return "Video Player";
        case "card-quiz": return "Card Quiz";
        case "drag-drop": return "Drag and Drop";
        default: return "";
    }
}

