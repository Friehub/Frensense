// SAFE: canActivateChild guard applied to parent with protected children

import { Injectable, NgModule } from '@angular/core'
import { RouterModule, type Routes, type CanActivateChild, type Router } from '@angular/router'
import { DashboardComponent } from './dashboard.component'

@Injectable({ providedIn: 'root' })
export class AuthChildGuard implements CanActivateChild {
  constructor(private router: Router) {}

  canActivateChild(): boolean {
    const token = localStorage.getItem('auth_token')
    if (!token) {
      this.router.navigate(['/login'])
      return false
    }
    return true
  }
}

const routes: Routes = [
  { path: 'login', component: () => import('./login.component') },
  {
    path: 'app',
    canActivateChild: [AuthChildGuard],
    children: [
      { path: 'dashboard', component: DashboardComponent },
      { path: 'profile', component: () => import('./profile.component') }
    ]
  }
]

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {}
