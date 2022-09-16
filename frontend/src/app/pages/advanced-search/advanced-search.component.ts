import { animate, state, style, transition, trigger } from '@angular/animations';
import { Component, OnInit } from '@angular/core';
import { CardSale } from 'src/app/models/card-sale.model';

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

  constructor() {
    this.setTransitionValues('left');
  }

  ngOnInit(): void {
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
  }

  public backToList() {
    this.setTransitionValues('right')
    this.pageState = 'search'
  }

}
