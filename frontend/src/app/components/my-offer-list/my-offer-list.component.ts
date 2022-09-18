import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { Sale } from 'src/app/models/sale.model';

@Component({
  selector: 'app-my-offer-list',
  templateUrl: './my-offer-list.component.html',
  styleUrls: ['./my-offer-list.component.sass']
})
export class MyOfferListComponent implements OnInit {

  @Output() private addOffer = new EventEmitter();
  @Output() private editSale = new EventEmitter<Sale>();
  @Output() private deleteSale = new EventEmitter<Sale>();
  @Input() public sales: Sale[] = [];

  constructor() { }

  ngOnInit(): void {

  }

  addOfferTrigger() {
    this.addOffer.emit();
  }

  editSaleTrigger(sale: Sale) {
    this.editSale.emit(sale);
  }

  deleteSaleTrigger(sale: Sale) {
    this.deleteSale.emit(sale);
  }
}
