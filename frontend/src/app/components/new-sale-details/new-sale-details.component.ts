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
    return this.isContactValid() &&
      this.isAmountValid() &&
      this.isPriceValid() &&
      this.isLocationValid();
  }

  public isAmountValid(): boolean {
    return (!!this.sale?.amount && this.sale?.amount > 0);
  }

  public isPriceValid(): boolean {
    return (!!this.sale && this.sale.price >= 0);
  }

  public isLocationValid(): boolean {
    return (this.sale?.contact_type == 'WEB') ||
      (!!this.sale?.location && this.sale?.location != "");
  }

  public isContactValid(): boolean {
    return (this.sale?.contact_type == 'EMAIL') ||
      (this.sale?.contact_type == 'PHONE') || 
      (this.sale?.contact_type == 'WEB' && !!this.sale?.web_address && this.sale?.web_address != '')
  }

  public submitSale() {
    if (!this.sale || !this.isInputValid()) {
      return;
    }
    this.sale.price = Math.round(this.sale.price * 100)
    this.data.postSale(this.sale).subscribe(resp => this.newSaleResp(resp))
  }

  private newSaleResp(resp: Sale): void {
    this.newSale.emit(resp);
  }
}
