import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, ParamMap, Router } from '@angular/router';
import { ToastrService } from 'ngx-toastr';
import { User } from 'src/app/models/user.model';
import { AuthService } from 'src/app/services/auth.service';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-token',
  templateUrl: './token.component.html',
  styleUrls: ['./token.component.sass']
})
export class TokenComponent implements OnInit {

  constructor(
    private auth: AuthService,
    private route: ActivatedRoute,
    private router: Router,
    private data: DataService,
    private toastr: ToastrService,
  ) { }

  ngOnInit(): void {
    // paramMap replaces params in Angular v4+
    this.route.paramMap.subscribe((params: ParamMap) => {
      const token = params.get('token');
      if (!token) {
        this.toastr.error("Neuspešen vpis", "Napaka!")
        return this.router.navigate([""]);
      }
      this.auth.login(token);
      this.toastr.success("Uspešen vpis", "")
      this.data.getUser().subscribe((data: User) => {
        sessionStorage.setItem("user", JSON.stringify(data));
        if (!data.name || !data.phone) {
          return this.router.navigate(["/register"]);
        }
        return this.router.navigate([""]);
      })
      return;
    });
  }

}

