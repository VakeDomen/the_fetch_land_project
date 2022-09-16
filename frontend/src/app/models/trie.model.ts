export class TrieTree<T> {
    children: Node<T>[];

    constructor() {
        this.children = [];
    }

    insertWord(word: string, obj: T, split?: boolean): void {
        console.log("TO INSERT: ", word);
        
        word = word.toUpperCase();
        if(word.length === 0) {
            return;
        }

        if (split) {
            const chunks = word.split(" ");
            if (chunks.length > 1) {
                for (let i = 1 ; i < chunks.length ; i++) {
                    let toInsert = "";
                    for (let j = i ; j < chunks.length ; j++) {
                        toInsert += " " + chunks[j]
                    }
                    this.insertWord(toInsert.trim(), obj);
                }
            }
        }

        let letterChild: Node<T> | null = null;
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

    collect(prefix: string): T[] {
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


class Node<T> {
    char: string;
    children: Node<T>[];
    leaf: boolean;
    obj: T[] | undefined;

    constructor(word: string, obj: T) {
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

    collect(prefix: string): T[] {
        const notes: T[] = [];
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

    insert(word: string, obj: T): void {
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