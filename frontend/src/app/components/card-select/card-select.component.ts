import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { Card } from 'src/app/models/card.model';
import { NewSale } from 'src/app/models/new-sale.model';
import { Sale } from 'src/app/models/sale.model';
import { TrieTree } from 'src/app/models/trie.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-card-select',
  templateUrl: './card-select.component.html',
  styleUrls: ['./card-select.component.sass']
})
export class CardSelectComponent implements OnInit {

  @Output() cardSelected = new EventEmitter<Card>();
  @Output() back = new EventEmitter();
  
  public cardLang: string = "en";
  public prefixQuery: string = "";
  public modalOpen: boolean = false;

  // autocomplete card search
  private tree: TrieTree = new TrieTree();
  selectedCard: Card | undefined;
  itemString: string = "";
  public cards: Card[] = [];


  constructor(
    private data: DataService,
  ) {}

  ngOnInit(): void {
    
  }

  fillTrie(cards: Card[]): void {
    this.cards = cards;
    this.tree = new TrieTree();
    for (const card of cards) {
      this.tree.insertWord(`${card.name} (${card.set_name})`, card);
    }
    console.log(this.tree);
    
  }

  public emitSelectedCard(card: Card) {
    this.cardSelected.emit(card);
  }

  public refreshCards() {
    console.log("refreshing")
    this.data.getCardsByPrefix(this.prefixQuery, this.cardLang).subscribe((cards: Card[]) => this.fillTrie(cards))
  }

  public search(value: string) {
    this.prefixQuery = value;
    if (this.prefixQuery.length == 2) {
      this.refreshCards()
    }
    this.cards = this.tree.collect(this.prefixQuery);
  }

  public backTrigger() {
    this.back.emit();
  }
}
