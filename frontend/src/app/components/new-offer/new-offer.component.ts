import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { Card } from 'src/app/models/card.model';
import { NewSale } from 'src/app/models/new-sale.model';
import { Sale } from 'src/app/models/sale.model';
import { TrieTree } from 'src/app/models/trie.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-new-offer',
  templateUrl: './new-offer.component.html',
  styleUrls: ['./new-offer.component.sass']
})
export class NewOfferComponent implements OnInit {

  @Output() saleCreated = new EventEmitter<Sale>();
  
  public sale: NewSale;
  public cardLang: string = "en";
  public prefixQuery: string = "";

  // autocomplete card search
  private tree: TrieTree = new TrieTree();
  selectedCard: Card | undefined;
  itemString: string = "";
  public cards: Card[] = [];
  
  private setBlacklist: string[] = [
    "token",
    "box"
  ];
  private setWhitelist: string[] = [
    "core",
    "expension",
    "commander",
    "masters",
    "funny",
    "box"
  ]

  constructor(
    private data: DataService,
  ) { 
    this.sale = {
      sale_type: "",
      sale_object_id: "",
      location_coords: "",
      description: "",
      price: 0,
      amount: 0,
    }
  }

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

  private emitSale(sale: Sale) {
    this.saleCreated.emit(sale);
  }

  public refreshCards() {
    console.log("refreshing")
    this.data.getCardsByPrefix(this.prefixQuery, this.cardLang).subscribe((cards: Card[]) => this.fillTrie(cards))
  }


  // autoselect stuff

  public itemSelectChange(value: any) {
    const items = this.tree.collect(value).filter((card: Card) => card.name == value );
    this.cards = items;
    
    if (items.length) {
      this.selectedCard = items[items.length - 1];
      this.sale.sale_object_id = this.selectedCard?.id ?? '';
      console.log("CARD SELECTED!", this.selectedCard);
    }
  }

  public search(value: string) {
    this.prefixQuery = value;
    if (this.prefixQuery.length == 2) {
      this.refreshCards()
    }
    this.cards = this.tree.collect(this.prefixQuery);
  }
}
