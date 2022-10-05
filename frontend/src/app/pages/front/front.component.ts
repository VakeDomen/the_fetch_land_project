import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Router } from '@angular/router';

@Component({
  selector: 'app-front',
  templateUrl: './front.component.html',
  styleUrls: ['./front.component.scss'],
})
export class FrontComponent implements OnInit {
  public searchQuery: string = '';

  constructor(private router: Router, private title: Title) {}

  ngOnInit(): void {
    this.title.setTitle('TheFethclandProject');
  }

  public search() {
    sessionStorage.setItem('saleSearchQeury', this.searchQuery);
    this.router.navigate(['search']);
  }
}
