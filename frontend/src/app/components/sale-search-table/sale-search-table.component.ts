import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { CardSale } from 'src/app/models/card-sale.model';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { TrieTree } from 'src/app/models/trie.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-sale-search-table',
  templateUrl: './sale-search-table.component.html',
  styleUrls: ['./sale-search-table.component.sass']
})
export class SaleSearchTableComponent implements OnInit {

  @Output() cardSelected = new EventEmitter<Card>();
  @Output() back = new EventEmitter();
  
  public cardLang: string = "en";
  public prefixQuery: string = "";

  // autocomplete card search
  private tree: TrieTree<CardSale> = new TrieTree();
  selectedCard: Card | undefined;
  itemString: string = "";
  public cardSales: CardSale[] = [];
  public cards: Card[] = [];


  constructor(
    private data: DataService,
  ) {}

  ngOnInit(): void {}
  
  public emitSelectedCard(card: Card) {
    this.cardSelected.emit(card);
  }

  public refreshCards() {
    this.data.getSalesByPrefix(this.prefixQuery, this.cardLang).subscribe((sales: Sale[]) => this.fillTrieSetup(sales))
  }

  private async fillTrieSetup(sales: Sale[]): Promise<void> {
    const cardPromises = sales.map((s: Sale) => {
      const card = this.data.getCardByIdFromTrie(s.sale_object_id);
      if (card.length) {
        return new Promise<Card>((resolve) => resolve(card.pop() as Card));
      } else {
        return this.data.getCardById(s.sale_object_id).toPromise() as Promise<Card>;
      }
    });
    const cards = await Promise.all(cardPromises);
    this.cards = cards;
    this.cards.map((c: Card) => this.data.insertCardsToTrie([c]));
    this.cardSales = sales.map((sale: Sale) => {
      const card = this.data.getCardByIdFromTrie(sale.sale_object_id).pop() as Card;
      const cs = sale as CardSale;
      cs.card = card;
      return cs;
    });
  }

  // private fillTrie(cardSales: CardSale[]): void {
  //   this.cardSales = cardSales;
  //   this.tree = new TrieTree();
  //   for (const cardSale of cardSales) {
  //     this.tree.insertWord(cardSale.card.name, cardSale);
  //   }
  //   this.data.insertCardsToTrie(cardSales.map(cs => cs.card));
  // }

  public search(value: string) {
    this.prefixQuery = value;
    if (this.prefixQuery.length == 2) {
      this.refreshCards()
    }
    this.cardSales = this.tree.collect(this.prefixQuery);
  }

  public backTrigger() {
    this.back.emit();
  }
}
