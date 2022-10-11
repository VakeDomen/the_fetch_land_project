import { animate, state, style, transition, trigger } from '@angular/animations';
import { Component, OnInit } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';
import { ActivatedRoute, ParamMap, Router } from '@angular/router';
import { CardSale } from 'src/app/models/card-sale.model';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-advanced-search',
  templateUrl: './advanced-search.component.html',
  styleUrls: ['./advanced-search.component.sass'],
  animations: [
    trigger('enter', [
      state('left', style({})),
      state('right', style({})),
      transition(':enter', [
        style({ transform: 'translateX({{ trans }}%)' }),
        animate('200ms'),
      ])
    ]),
  ]  
})
export class AdvancedSearchComponent implements OnInit {

  public pageState: 'search' | 'sales' = 'search';
  public salesToCheck: CardSale[] = [];
  private transitionXValue = 200;
  public transitionDirection: 'left' | 'right' = 'left';
  public transitionEnter = 0;
  public transitionLeave = 0;

  constructor(
    private route: ActivatedRoute,
    private data: DataService,
    private title: Title,
    private router: Router,
    private meta: Meta,
  ) {
    this.setTransitionValues('left');
    this.title.setTitle("Iskanje kart | TheFethclandProject");
    this.meta.updateTag({name: "description", content: "Naprednejše iskanje ponudb kart."});
    this.route
      .queryParamMap
      .subscribe((params: ParamMap) => {
        const queryId = params.get('id');
        if (queryId) {
          this.setupPreQuery(queryId)
        }
      }
    );
  }

  ngOnInit(): void {
  }

  private setupPreQuery(cardId: string) {
    this.data.getCardById(cardId).subscribe((card: Card) => {
      this.data.insertCardsToTrie([card]);
      this.data.getCardSalesById(cardId).subscribe((sales: Sale[]) => {
        this.checkSales(this.generateCardSales(cardId, sales));
      })
    })
  }

  private generateCardSales(cardId: string, sales: Sale[]): CardSale[] {
    const card = this.data.getCardByIdFromTrie(cardId).pop();
    if (!card) {
      return [];
    }
    return sales.map((s: Sale) => {
      const cardSale = s as CardSale;
      cardSale.card = card;
      return cardSale;
    })
  }

  private setTransitionValues(value: 'left' | 'right') {
    this.transitionDirection = value;
    this.transitionEnter = (value == 'left' ? this.transitionXValue : -this.transitionXValue);
    this.transitionLeave = (value == 'left' ? -this.transitionXValue : this.transitionXValue);
  }

  public checkSales(cardSales: CardSale[]) {
    this.salesToCheck = cardSales;
    this.setTransitionValues('left');
    this.pageState = 'sales'
    this.router.navigate(['.'], { relativeTo: this.route, queryParams: { id: this.salesToCheck[0].card.id }});
  }

  public backToList() {
    this.setTransitionValues('right')
    this.pageState = 'search'
    this.router.navigate(['.'], { relativeTo: this.route, queryParams: { name: this.salesToCheck[0].card.name }});
  }

}
