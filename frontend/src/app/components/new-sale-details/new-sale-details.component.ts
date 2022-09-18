import { Component, EventEmitter, Input, OnChanges, OnInit, Output, SimpleChanges } from '@angular/core';
import { ToastrService } from 'ngx-toastr';
import { Card } from 'src/app/models/card.model';
import { SaleEdit } from 'src/app/models/sale-edit.model';
import { Sale } from 'src/app/models/sale.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-new-sale-details',
  templateUrl: './new-sale-details.component.html',
  styleUrls: ['./new-sale-details.component.sass']
})
export class NewSaleDetailsComponent implements OnChanges {

  @Input() card: Card | undefined;
  @Input() sale: Sale | undefined;
  @Input() editMode: boolean = false;

  @Output() back = new EventEmitter();
  @Output() newSale = new EventEmitter<Sale>();
  @Output() editedSale = new EventEmitter<Sale>();

  public price: number = 0;

  constructor(
    private data: DataService,
    private toastr: ToastrService,
  ) { }
  ngOnChanges(changes: SimpleChanges): void {
    if (this.sale) {
      this.price = this.sale.price;
      this.price /= 100;
    }
  }

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

  public finnish() {
    if (this.editMode) this.editSale();
    else this.submitSale();
  }

  private submitSale() {
    if (!this.sale || !this.isInputValid()) {
      return;
    }
    this.sale.price = this.price;
    this.sale.price = Math.round(this.sale.price * 100)
    this.data.postSale(this.sale).subscribe(resp => this.newSaleResp(resp))
  }

  private editSale() {
    if (!this.sale || !this.isInputValid()) {
      return;
    }
    this.sale.price = this.price;
    this.sale.price = Math.round(this.sale.price * 100)
    const saleEdit = this.saleToEditPost(this.sale)
    this.data.postSaleEdit(this.sale.id, saleEdit).subscribe(resp => this.editSaleResp(resp))
  }

  private newSaleResp(resp: Sale): void {
    this.toastr.success("Uspešno dodana ponudba", "");
    this.newSale.emit(resp);
  }

  private editSaleResp(resp: Sale): void {
    this.toastr.success("Uspešno urejena ponudba", "");
    this.editedSale.emit(resp);
  }

  private saleToEditPost(sale: Sale): SaleEdit {
    return {
      description: sale.description,
      price: sale.price,
      amount: sale.amount,
      contact_type: sale.contact_type,
      location: sale.location,
      web_address: sale.web_address,
    } as SaleEdit;
  }
}
