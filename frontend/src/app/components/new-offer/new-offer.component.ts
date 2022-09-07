import { Component, EventEmitter, OnInit, Output } from '@angular/core';
import { Sale } from 'src/app/models/sale.model';

@Component({
  selector: 'app-new-offer',
  templateUrl: './new-offer.component.html',
  styleUrls: ['./new-offer.component.sass']
})
export class NewOfferComponent implements OnInit {

  @Output() saleCreated = new EventEmitter<Sale>();
  
  public sale: Sale;

  constructor() { 
    this.sale = {
      id: "",
      sale_type: "",
      user_id: "",
      sale_object_id: "",
      location_coords: "",
      created: "",
      description: "",
      price: 0,
      amount: 0,
    }
  }

  ngOnInit(): void {
  }


  private emitSale() {
    this.saleCreated.emit(this.sale);
  }
}
