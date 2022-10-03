import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { Card } from 'src/app/models/card.model';
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
  private maxAmountOfCardsToDisplayOnSearch: number = 20;

  // autocomplete card search
  private tree: TrieTree<Card> = new TrieTree();
  public selectedCard: Card | undefined;
  public itemString: string = "";
  public cards: Card[] = [];
  public cardsToShow: Card[] = [];
  public searching: boolean = false;
  private shouldSearch: boolean = false;

  constructor(
    private data: DataService,
  ) {}

  ngOnInit(): void {
    
  }

  fillTrie(cards: Card[]): void {
    this.tree = new TrieTree();
    for (const card of cards) {
      this.tree.insertWord(`${card.name} (${card.set_name})`, card, false);
    }
    this.data.insertCardsToTrie(cards);
    this.searching = false;
    this.shouldSearch = false;
    this.cards = cards;
    this.cards.sort((c1: Card, c2: Card) => c1.name.length - this.prefixQuery.length > c2.name.length - this.prefixQuery.length ? 1 : -1);
    this.cardsToShow = [...this.cards.slice(0, this.maxAmountOfCardsToDisplayOnSearch)]
  }

  public emitSelectedCard(card: Card) {
    this.cardSelected.emit(card);
  }

  public refreshCards() {
    this.shouldSearch = true;
    setTimeout(() => {if (this.shouldSearch) {this.searching = true}}, 100); 
    this.data.getCardsByPrefix(this.prefixQuery, this.cardLang).subscribe((cards: Card[]) => this.fillTrie(cards))
  }

  public search(value: string) {
    this.prefixQuery = value;
    if (this.prefixQuery.length > 1) this.refreshCards();
    else this.cards = this.tree.collect(this.prefixQuery);
  }

  public backTrigger() {
    this.back.emit();
  }
}
