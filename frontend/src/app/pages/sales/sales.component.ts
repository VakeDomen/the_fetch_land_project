import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { animate, query, stagger, state, style, transition, trigger } from '@angular/animations';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { User } from 'src/app/models/user.model';
import { AuthService } from 'src/app/services/auth.service';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-sales',
  templateUrl: './sales.component.html',
  styleUrls: ['./sales.component.sass'],
  animations: [
    trigger('enter', [
      state('left', style({})),
      state('right', style({})),
      transition(':enter', [
        style({ transform: 'translateX({{ trans }}%)' }),
        animate('200ms'),
      ])
    ]),
  ]
})
export class SalesComponent implements OnInit {

  public sales: Sale[] = [];
  public newSale: Sale | undefined;
  public newSaleCard: Card | undefined;
  public saleToEdit: Sale | undefined;
  public saleToEditCard: Card | undefined;

  public pageState: 'list' | 'search' | 'detalis' | 'edit' = 'list';
  private transitionXValue = 200;
  public transitionDirection: 'left' | 'right' = 'left';
  public transitionEnter = 0;
  public transitionLeave = 0;
  

  constructor(
    private data: DataService,
    private auth: AuthService,
    private router: Router,
  ) {
    this.setTransitionValues('right');
  }

  ngOnInit(): void {
    this.setTransitionValues('left');
    this.data.getUserSales().subscribe((sales: Sale[]) => this.saveSales(sales));
  }

  private setTransitionValues(value: 'left' | 'right') {
    this.transitionDirection = value;
    this.transitionEnter = (value == 'left' ? this.transitionXValue : -this.transitionXValue);
    this.transitionLeave = (value == 'left' ? -this.transitionXValue : this.transitionXValue);
  }

  private saveSales(sales: Sale[]): void {
    this.sales = sales;
  }

  public startSearch() {
    this.checkContactData();
    this.setTransitionValues('left');
    this.pageState = 'search';
  }

  public backToList() {
    this.setTransitionValues('right');
    this.pageState = 'list'
  }

  public backToSearch() {
    this.setTransitionValues('right');
    this.pageState = 'search'
  }

  private checkContactData() {
    const userString = sessionStorage.getItem('user');
    // not logged in
    if (!userString) {
      return this.router.navigate(['']);
    }
    const user = JSON.parse(userString) as User;
    if (!user.phone || !user.name || user.phone == '' || user.name == '') {
      return this.router.navigate(["profile"]);
    }
    return;
  }

  public nextToDetails(card: Card): void {
    this.setTransitionValues('left');
    this.newSaleCard = card
    this.newSale = {
      sale_type: "CARD",
      sale_object_id: card.id,
      description: "",
      price: 0,
      amount: 0,
      contact_type: 'PHONE',
      location: '',
      web_address: '',
    } as Sale;
    this.pageState = 'detalis';
  }

  public newSaleSubmitted(sale: Sale) {
    // sale.price /= 100;
    this.sales.push(sale);
    this.setTransitionValues('left');
    this.pageState = 'list';
  }

  public saleEdited(sale: Sale) {
    this.setTransitionValues('left');
    this.pageState = 'list';
  }

  public editSaleTrigger(sale: Sale) {
    this.saleToEditCard = this.data.getCardByIdFromTrie(sale.sale_object_id).pop();
    this.saleToEdit = sale;
    this.setTransitionValues('left');
    this.pageState = 'edit';
  }

  public deleteSaleTrigger(sale: Sale) {
    
  }
}
