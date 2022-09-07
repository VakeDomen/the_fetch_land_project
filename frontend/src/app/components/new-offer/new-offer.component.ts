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

  // autocomplete card search
  private tree: TrieTree = new TrieTree();
  public results: string[] = [];
  selectedCard: Card | undefined;
  itemString: string = "";
  itemNum: number = 1;

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
    this.tree = new TrieTree();
    for (const card of cards) {
      this.tree.insertWord(card.name, card);
    }
    console.log(this.tree);
    
  }

  private emitSale(sale: Sale) {
    this.saleCreated.emit(sale);
  }


  // autoselect stuff

  public itemSelectChange(value: any) {
    const item = this.tree.collect(value);
    if (item.length == 1) {
      this.selectedCard = item.pop();
      this.sale.sale_object_id = this.selectedCard?.id ?? '';
      console.log("CARD SELECTED!", this.selectedCard);
    }
  }

  public search(event: any) {
    console.log(event);
    if (event.query.length && event.query.length == 3) {
      this.data.getCardsByPrefix(event.query).subscribe((cards: Card[]) => this.fillTrie(cards))
    }
    this.results = this.tree.collect(event.query).map((item: Card) => item.name);
    console.log(this.results);
    
  }
}
