import { Component, Input, OnChanges, OnInit, SimpleChanges } from '@angular/core';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-my-offers-table',
  templateUrl: './my-offers-table.component.html',
  styleUrls: ['./my-offers-table.component.sass']
})
export class MyOffersTableComponent implements OnChanges {

  @Input() sales: Sale[] = [];

  cardCounter: number = 0;
  dataReady: boolean = false;

  constructor(
    private data: DataService,
  ) { }

  ngOnChanges(changes: SimpleChanges): void {
    for (const sale of this.sales) {
      this.data.getCardById(sale.sale_object_id).subscribe((cards: Card) => this.handleFetchResponse(cards));
    }
  }
  
  handleFetchResponse(card: Card): void {
    this.cardCounter++;
    this.data.insertCardsToTrie([card]);
    this.dataReady = this.sales.length == this.cardCounter;
  }

  getCard(id: string) {
    return this.data.getCardByIdFromTrie(id).pop()
  }
}
