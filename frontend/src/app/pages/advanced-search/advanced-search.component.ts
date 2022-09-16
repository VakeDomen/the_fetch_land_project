import { Component, OnInit } from '@angular/core';
import { CardSale } from 'src/app/models/card-sale.model';

@Component({
  selector: 'app-advanced-search',
  templateUrl: './advanced-search.component.html',
  styleUrls: ['./advanced-search.component.sass']
})
export class AdvancedSearchComponent implements OnInit {

  public pageState: 'search' | 'sales' = 'search';
  public salesToCheck: CardSale[] = [];

  constructor() { }

  ngOnInit(): void {
  }

  public checkSales(cardSales: CardSale[]) {
    this.salesToCheck = cardSales;
    this.pageState = 'sales'
  }

}
