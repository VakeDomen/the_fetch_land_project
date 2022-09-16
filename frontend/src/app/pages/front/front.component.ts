import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';

@Component({
  selector: 'app-front',
  templateUrl: './front.component.html',
  styleUrls: ['./front.component.sass']
})
export class FrontComponent implements OnInit {

  public searchQuery: string = '';

  constructor(
    private router: Router,
  ) { }

  ngOnInit(): void {
  }

  public search() {
    sessionStorage.setItem("saleSearchQeury", this.searchQuery);
    this.router.navigate(["search"]);
  }
}
