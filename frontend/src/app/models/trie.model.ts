import { Card } from "./card.model";

export class TrieTree {
    children: Node[];

    constructor() {
        this.children = [];
    }

    insertWord(word: string, obj: Card): void {
        word = word.toUpperCase();
        if(word.length === 0) {
            return;
        }
        let letterChild: Node | null = null;
        for (const child of this.children) {
            if (child.char === word.charAt(0)) {
                letterChild = child;
                break;
            }
        }
        if (letterChild === null) {
            letterChild = new Node(word, obj);
            this.children.push(letterChild);
        } else {
            letterChild.insert(word, obj);
        }
    }

    collect(prefix: string): Card[] {
        prefix = prefix.toUpperCase();
        if(!prefix || prefix.length === 0 || this.children.length === 0) {
            return [];
        }
        //traverse
        for (const child of this.children) {
            if(child.char === prefix.charAt(0)) {
                return child.collect(prefix.substr(1));
            }
        }
        return [];
    }
}


class Node {
    char: string;
    children: Node[];
    leaf: boolean;
    obj: Card[] | undefined;

    constructor(word: string, obj: Card) {
        this.char = word.charAt(0);
        this.children = [];
        if(word.length === 1) {
            this.leaf = true;
            if (!this.obj) {
                this.obj = [];
            }
            this.obj.push(obj);
        } else {
            this.leaf = false;
            this.children.push(new Node(word.substr(1), obj));
        }
    }

    collect(prefix: string): Card[] {
        const notes: Card[] = [];
        if (prefix.length === 0) {
            //collect
            if (this.leaf && this.obj) {
                notes.push(...this.obj);
            }
            if (this.children.length === 0) {
                return notes;
            }            
            for (const child of this.children) {
                notes.push(...child.collect(""));
            }
            return notes;
        } else {
            //traverse    
            if(this.children.length === 0) {
                return notes;
            }
            for (const child of this.children) {
                if(child.char === prefix.charAt(0)) {
                    notes.push(...child.collect(prefix.substr(1)));
                }
            }
            return notes;
        }
    }

    insert(word: string, obj: Card): void {
        if (word.length === 1) {
            this.leaf = true;
            if (!this.obj) {
                this.obj = [];
            }
            this.obj.push(obj);
            return;
        }
        word = word.substr(1); 
        if (this.children.length > 0) {
            for (const child of this.children) {
                if (child.char === word.charAt(0)) {
                    child.insert(word, obj);
                    return;
                }
            }
        }
        this.children.push(new Node(word, obj));
    }
}