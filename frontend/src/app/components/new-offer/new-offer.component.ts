import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { NewSale } from 'src/app/models/new-sale.model';
import { Sale } from 'src/app/models/sale.model';

@Component({
  selector: 'app-new-offer',
  templateUrl: './new-offer.component.html',
  styleUrls: ['./new-offer.component.sass']
})
export class NewOfferComponent implements OnInit {

  @Output() saleCreated = new EventEmitter<Sale>();
  
  public sale: NewSale;

  constructor() { 
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


  private emitSale(sale: Sale) {
    this.saleCreated.emit(sale);
  }
}
