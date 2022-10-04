import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute, ParamMap } from '@angular/router';
import { CardSale } from 'src/app/models/card-sale.model';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { TrieTree } from 'src/app/models/trie.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-sale-search',
  templateUrl: './sale-search.component.html',
  styleUrls: ['./sale-search.component.sass']
})
export class SaleSearchComponent implements OnInit {
  @Output() cardSelected = new EventEmitter<Card>();
  @Output() back = new EventEmitter();
  @Output() salesToCheck = new EventEmitter<CardSale[]>();

  
  public cardLang: string = "en";
  public prefixQuery: string = "";

  // autocomplete card search
  private tree: TrieTree<CardSale> = new TrieTree();
  selectedCard: Card | undefined;
  itemString: string = "";
  public cardSales: CardSale[] = [];
  public searching: boolean = false;
  private shouldSearch: boolean = false;


  constructor(
    private data: DataService,
    private route: ActivatedRoute,
    private title: Title,
  ) {}

  ngOnInit(): void {
    this.title.setTitle("Iskanje kart | TheFethclandProject");
    this.route
      .queryParamMap
      .subscribe((params: ParamMap) => {
        const queryName = params.get('name');
        if (queryName) {
          sessionStorage.setItem('saleSearchQeury', queryName);
        }
        this.setupPreQuery()
      }
    );
  }

  private setupPreQuery() {
    const query = sessionStorage.getItem("saleSearchQeury");
    if (query) {
      this.prefixQuery = query;
      this.refreshCardsPartials();
    }
  }
  
  public emitSelectedCard(card: Card) {
    this.cardSelected.emit(card);
  }

  public refreshCards() {
    this.data.getSalesByPrefix(this.prefixQuery, this.cardLang).subscribe((sales: Sale[]) => this.fillTrieSetup(sales))
  }

  public refreshCardsPartials() {
    this.shouldSearch = true;
    setTimeout(() => {if (this.shouldSearch) {this.searching = true}}, 100); 
    this.data.getSalesByPartialPrefix(this.prefixQuery, this.cardLang).subscribe((sales: Sale[]) => this.fillTrieSetup(sales))
  }

  private async fillTrieSetup(sales: Sale[]): Promise<void> {
    const cardSales = await this.generateCardSaleObjects(sales);    
    this.fillTrie(cardSales)
    this.cardSales = cardSales;
    this.shouldSearch = false;
    this.searching = false;
  }

  private async generateCardSaleObjects(sales: Sale[]) {
    const cardPromises = sales.map((s: Sale) => {
      const card = this.data.getCardByIdFromTrie(s.sale_object_id);
      if (card.length) {
        return new Promise<Card>((resolve) => resolve(card.pop() as Card));
      } else {
        return this.data.getCardById(s.sale_object_id).toPromise() as Promise<Card>;
      }
    });
    const cards = await Promise.all(cardPromises);
    this.data.insertCardsToTrie(cards);
    return sales.map((sale: Sale) => {
      const card = this.data.getCardByIdFromTrie(sale.sale_object_id).pop() as Card;
      const cs = sale as CardSale;
      cs.card = card;
      return cs;
    });
  }

  private fillTrie(cardSales: CardSale[]): void {
    this.cardSales = cardSales;
    for (const cardSale of cardSales) {
      this.tree.insertWord(cardSale.card.name, cardSale, false);
    }
  }

  public search() {
    sessionStorage.setItem("saleSearchQeury", this.prefixQuery);
    if (this.prefixQuery.length > 1) this.refreshCardsPartials()
    else this.cardSales = this.tree.collect(this.prefixQuery);
  }

  public backTrigger() {
    this.back.emit();
  }

  public checkSales(sales: CardSale[]) {
    this.salesToCheck.emit(sales);
  }
}
