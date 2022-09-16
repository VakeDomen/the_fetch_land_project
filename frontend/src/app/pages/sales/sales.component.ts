import { Component, OnInit } from '@angular/core';
import { CardSelectComponent } from 'src/app/components/card-select/card-select.component';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-sales',
  templateUrl: './sales.component.html',
  styleUrls: ['./sales.component.sass']
})
export class SalesComponent implements OnInit {

  public sales: Sale[] = [];
  public newSale: Sale | undefined;
  public newSaleCard: Card | undefined;

  public pageState: 'list' | 'search' | 'detalis' = 'list'

  constructor(
    private data: DataService,
  ) { }

  ngOnInit(): void {
    this.data.getUserSales().subscribe((sales: Sale[]) => this.saveSales(sales));
  }

  private saveSales(sales: Sale[]): void {
    sales = sales.map((s: Sale) => {s.price /= 100; return s});
    this.sales = sales;
  }

  public nextToDetails(card: Card): void {
    this.newSaleCard = card
    this.newSale = {
      sale_type: "CARD",
      sale_object_id: card.id,
      description: "",
      price: 0,
      amount: 0,
      contact_type: 'EMAIL',
      location: '',
      web_address: '',
    } as Sale;
    this.pageState = 'detalis';
  }

  public newSaleSubmitted(sale: Sale) {
    this.sales.push(sale);
    this.pageState = 'list';
  }
}
