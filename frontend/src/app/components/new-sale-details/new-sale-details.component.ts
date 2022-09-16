import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-new-sale-details',
  templateUrl: './new-sale-details.component.html',
  styleUrls: ['./new-sale-details.component.sass']
})
export class NewSaleDetailsComponent implements OnInit {

  @Input() card: Card | undefined;
  @Input() sale: Sale | undefined;

  @Output() back = new EventEmitter();
  @Output() newSale = new EventEmitter<Sale>()

  constructor(
    private data: DataService,
  ) { }

  ngOnInit(): void {}

  public backTrigger() {
    this.back.emit();
  }

  public isInputValid(): boolean {
    return this.isAmountValid() &&
      this.isPriceValid() &&
      this.isLocationValid();
  }

  public isAmountValid(): boolean {
    if (!this.sale?.amount || this.sale?.amount <= 0) {
      return false;
    }
    return true;
  }

  public isPriceValid(): boolean {
    if (!this.sale || this.sale?.price < 0) {
      return false;
    }
    return true;
  }

  public isLocationValid(): boolean {
    if (!this.sale?.location || this.sale?.location === "") {
      return false;
    }
    return true;
  }

  public submitSale() {
    if (!this.sale || !this.isInputValid()) {
      return;
    }
    this.sale.price = Math.round(this.sale.price * 100)
    this.data.postSale(this.sale).subscribe(resp => this.newSaleResp(resp))
  }

  private newSaleResp(resp: Sale): void {
    this.newSale.emit(this.sale);
  }
}
