import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute, ParamMap, Router } from '@angular/router';
import { CardSale } from 'src/app/models/card-sale.model';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { TrieTree } from 'src/app/models/trie.model';
import { DataService } from 'src/app/services/data.service';
import { SessionService } from 'src/app/services/session.service';

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
  private tree: TrieTree<CardSale> = new TrieTree();
  public selectedCard: Card | undefined;
  public cardSales: CardSale[] = [];
  public searching: boolean = false;
  private shouldSearch: boolean = false;

  constructor(
    private data: DataService,
    private route: ActivatedRoute,
    private title: Title,
    private router: Router,
    private sessionStorage: SessionService,
  ) {
    this.title.setTitle("Iskanje kart | TheFethclandProject");
    let triggers = 0;
    this.route.queryParamMap.subscribe((params: ParamMap) => {
      if (triggers == 0) {
        const queryName = params.get('name');
        if (queryName) {
          this.sessionStorage.setItem('saleSearchQeury', queryName);
        }
        this.setupPreQuery()
      }
      triggers++;
    });
  }

  ngOnInit(): void {
  }

  private setupPreQuery() {
    const query = this.sessionStorage.getItem("saleSearchQeury");
    if (query) {
      this.prefixQuery = query;
      this.refreshCardsPartials(this.prefixQuery);
    }
  }

  public emitSelectedCard(card: Card) {
    this.cardSelected.emit(card);
  }

  public refreshCards() {
    this.data.getSalesByPrefix(this.prefixQuery, this.cardLang).subscribe((sales: Sale[]) => this.fillTrieSetup(sales, this.prefixQuery))
  }

  public refreshCardsPartials(previousQueryPrefix: string) {
    this.shouldSearch = true;
    setTimeout(() => { if (this.shouldSearch) { this.searching = true } }, 100);
    this.data.getSalesByPartialPrefix(this.prefixQuery, this.cardLang).subscribe((sales: Sale[]) => this.fillTrieSetup(sales, previousQueryPrefix))
  }

  private async fillTrieSetup(sales: Sale[], previousQueryPrefix: string): Promise<void> {
    const cardSales = await this.generateCardSaleObjects(sales);
    this.fillTrie(cardSales)
    if (this.prefixQuery == previousQueryPrefix) {
      this.cardSales = cardSales;
    }
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
    for (const cardSale of cardSales) {
      this.tree.insertWord(cardSale.card.name, cardSale, false);
    }
  }

  public search() {
    this.sessionStorage.setItem("saleSearchQeury", this.prefixQuery);
    this.router.navigate(['.'], { relativeTo: this.route, queryParams: { name: this.prefixQuery } });
    if (this.prefixQuery.length > 1) {
      // clone variable
      let varFreeze: string = JSON.parse(JSON.stringify(this.prefixQuery));
      setTimeout(() => this.conditionalRefresh(varFreeze), 200);
    } else this.cardSales = this.tree.collect(this.prefixQuery);
  }

  private conditionalRefresh(previousQueryPrefix: string) {
    if (this.prefixQuery == previousQueryPrefix) {
      this.refreshCardsPartials(previousQueryPrefix);
    }
    
  }

  public backTrigger() {
    this.back.emit();
  }

  public checkSales(sales: CardSale[]) {
    this.salesToCheck.emit(sales);
  }
}
