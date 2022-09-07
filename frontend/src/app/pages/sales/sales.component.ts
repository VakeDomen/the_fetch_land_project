import { Component, OnInit } from '@angular/core';
import { Sale } from 'src/app/models/sale.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-sales',
  templateUrl: './sales.component.html',
  styleUrls: ['./sales.component.sass']
})
export class SalesComponent implements OnInit {

  public sales: Sale[] = [];
  public salesInTheMaking: Sale[] = [];

  constructor(
    private data: DataService,
  ) { }

  ngOnInit(): void {
    this.data.getUserSales().subscribe((sales: Sale[]) => this.saveSales(sales));
  }

  private saveSales(sales: Sale[]): void {
    console.log(sales);
    
    this.sales = sales;
  }

  public removeSale(event: any, offerIndex: number): void {
    console.log(event);
    
  }

  public addOffer(): void {
    this.salesInTheMaking.push({
      id: '',
      sale_type: '',
      user_id: '',
      sale_object_id: '',
      location_coords: '',
      created: '',
      description: '',
      price: 0,
    } as Sale)
  }

}
