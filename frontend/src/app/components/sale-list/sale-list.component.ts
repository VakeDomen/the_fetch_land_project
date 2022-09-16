import { Component, EventEmitter, Input, OnChanges, Output, SimpleChanges } from '@angular/core';
import { CardSale } from 'src/app/models/card-sale.model';
import { Card } from 'src/app/models/card.model';

@Component({
  selector: 'app-sale-list',
  templateUrl: './sale-list.component.html',
  styleUrls: ['./sale-list.component.sass']
})
export class SaleListComponent implements OnChanges {

  @Input() cardSales: CardSale[] = [];
  @Output() back = new EventEmitter();

  public card: Card | undefined;

  constructor() { }
  ngOnChanges(changes: SimpleChanges): void {
    if (this.cardSales.length) {
      this.card = this.cardSales[0].card;
    }
  }

  public backTrigger() {
    this.back.emit();
  }

}
