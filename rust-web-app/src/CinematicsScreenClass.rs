// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CinematicsScreenClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Imaging;
// usingSystem.IO;

namespace WindowsApplication1
{
  pub class CinematicsScreenClass : ScreenClass
  {
     DateTime lasttime;
     DateTime lastfadetime;
     int pagespeed;
     int fadespeed;
     int pagenr;
     int lastpagenr;
     int fadenr;
     int fadeFrameCap;
     bmp: Bitmap;
     lastbmp: Bitmap;

    pub CinematicsScreenClass(ref tGame: GameClass, tformref: Form1)
      : base(ref tGame, tFormRef: tformref)
    {
      this.lasttime = DateAndTime.Now;
      this.pagespeed = 500;
      this.fadeFrameCap = 14;
      this.fadespeed = 30;
      this.pagenr = 0;
      this.lastpagenr = 20;
      if (this.Game.EditObj.IntroSoundOn)
        SoundMod.PlayEventBackground(this.Game.AppPath + "sound/" + this.Game.ModOpeningSoundtrack, ref this.Game.EditObj);
      this.loadPageStuff(this.pagenr);
    }

    pub Paint: Bitmap(bool onlyToolTip = false)
    {
      Graphics Expression = Graphics.FromImage((Image) this.OwnBackground);
      if (!Information.IsNothing( this.bmp))
      {
        let mut screenWidth: i32 =  this.Game.ScreenWidth;
        let mut num1: i32 =  (int) Math.Round(16.0 / 9.0 *  this.Game.ScreenHeight);
        let mut num2: i32 =  (int) Math.Round(9.0 / 16.0 *  screenWidth);
        let mut width: i32 =  num1 + 1;
        let mut height: i32 =  num2 + 1;
        if (width < this.Game.ScreenWidth)
        {
          height = (int) Math.Round( height * ( this.Game.ScreenWidth /  width));
          width = this.Game.ScreenWidth;
        }
        if (height < this.Game.ScreenHeight)
        {
          width = (int) Math.Round( width * ( this.Game.ScreenHeight /  height));
          height = this.Game.ScreenHeight;
        }
        int x;
        if (width > this.Game.ScreenWidth + 1)
        {
          int num3;
          x = (int) Math.Round( num3 -  (width - this.Game.ScreenWidth) / 2.0);
        }
        int y;
        if (height > this.Game.ScreenHeight + 1)
        {
          int num4;
          y = (int) Math.Round( num4 -  (height - this.Game.ScreenHeight) / 2.0);
        }
        if (!Information.IsNothing( this.lastbmp) & this.fadenr >= 1 & this.fadenr < this.fadeFrameCap)
        {
          ref Graphics local1 = ref Expression;
          ref local2: Bitmap = ref this.lastbmp;
          Rectangle rectangle1 = Rectangle::new(0, 0, 1920, 1080);
          let mut srcrect1: &Rectangle = &rectangle1
          Rectangle rectangle2 = Rectangle::new(x, y, width, height);
          let mut destrect1: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2(ref local1, ref local2, srcrect1, destrect1);
          ref Graphics local3 = ref Expression;
          ref local4: Bitmap = ref this.bmp;
          rectangle2 = Rectangle::new(0, 0, 1920, 1080);
          let mut srcrect2: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y, width, height);
          let mut destrect2: &Rectangle = &rectangle1
          double alpha =  ( this.fadenr /  this.fadeFrameCap);
          DrawMod.DrawSimplePartAlpha(ref local3, ref local4, srcrect2, destrect2,  alpha);
        }
        else
          DrawMod.DrawSimplePart2(ref Expression, ref this.bmp, Rectangle::new(0, 0, 1920, 1080), Rectangle::new(x, y, width, height));
      }
      else
        Expression.Clear(Color.Black);
      let mut num: i32 =  (int) Math.Round( (this.Game.ScreenHeight - 768) / 2.0 + 100.0);
      if (this.pagenr == 1)
        this.pagespeed = 1200;
      if (this.pagenr == 2)
      {
        this.introText(ref Expression, "Once upon a time...", 1, num + 0);
        this.pagespeed = 2000;
      }
      if (this.pagenr == 3)
      {
        this.introText(ref Expression, "Once upon a time...", 1, num + 0);
        this.introText(ref Expression, "A Galactic Republic united tens of thousands of stars", 2, num + 0);
        this.pagespeed = 4500;
      }
      if (this.pagenr == 4)
        this.pagespeed = 2500;
      if (this.pagenr == 5)
      {
        this.introText(ref Expression, "Once upon a time...", 1, num + 0);
        this.pagespeed = 2000;
      }
      if (this.pagenr == 6)
      {
        this.introText(ref Expression, "Once upon a time...", 1, num + 0);
        this.introText(ref Expression, "An interstellar senate reigned over millenia of peace and tranquility", 2, num + 0);
        this.pagespeed = 4500;
      }
      if (this.pagenr == 7)
        this.pagespeed = 2500;
      if (this.pagenr == 8)
      {
        this.introText(ref Expression, "Until...", 1, num + 0);
        this.pagespeed = 2000;
      }
      if (this.pagenr == 9)
      {
        this.introText(ref Expression, "Until...", 1, num + 0);
        this.introText(ref Expression, "The Dissolution War destroyed the star lanes and the high economy", 2, num + 0);
        this.pagespeed = 4500;
      }
      if (this.pagenr == 10)
        this.pagespeed = 2500;
      if (this.pagenr == 11)
      {
        this.introText(ref Expression, "It was a time...", 1, num + 0);
        this.pagespeed = 2000;
      }
      if (this.pagenr == 12)
      {
        this.introText(ref Expression, "It was a time...", 1, num + 0);
        this.introText(ref Expression, "Where each planet found itself isolated and alone", 2, num + 0);
        this.pagespeed = 4500;
      }
      if (this.pagenr == 13)
        this.pagespeed = 2500;
      if (this.pagenr == 14)
      {
        this.introText(ref Expression, "It was a time...", 1, num + 0);
        this.pagespeed = 2000;
      }
      if (this.pagenr == 15)
      {
        this.introText(ref Expression, "It was a time...", 1, num + 0);
        this.introText(ref Expression, "Where chaos and anarchy ripped apart civilisation", 2, num + 0);
        this.pagespeed = 4500;
      }
      if (this.pagenr == 16)
        this.pagespeed = 2500;
      if (this.pagenr == 17)
      {
        this.introText(ref Expression, "Now...", 1, num + 0);
        this.pagespeed = 2000;
      }
      if (this.pagenr == 18)
      {
        this.introText(ref Expression, "Now...", 1, num + 0);
        this.introText(ref Expression, "New leaders are rising", 2, num + 0);
        this.pagespeed = 2500;
      }
      if (this.pagenr == 19)
      {
        this.introText(ref Expression, "Now...", 1, num + 0);
        this.introText(ref Expression, "New leaders are rising", 2, num + 0);
        this.introText(ref Expression, "Eager for conquest", 2, num + 100);
        this.pagespeed = 2500;
      }
      if (this.pagenr == 20)
      {
        this.introText(ref Expression, "Now...", 1, num + 0);
        this.introText(ref Expression, "New leaders are rising", 2, num + 0);
        this.introText(ref Expression, "Eager for conquest", 2, num + 100);
        this.introText(ref Expression, "Eager to re-unite humanity", 2, num + 200);
        this.pagespeed = 4500;
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return this.OwnBackground;
    }

    pub fn introText(ref Graphics g, s: String, int mode, int plusy)
    {
      let mut x: i32 =  (int) Math.Round( this.Game.ScreenWidth / 2.0);
      let mut num: i32 =  (int) Math.Round( this.Game.ScreenHeight / 2.0);
      if (mode == 1)
      {
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont1, x + 1, 30 + plusy, Color.FromArgb(0, 0, 0, 58));
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont1, x + 0, 31 + plusy, Color.FromArgb(0, 0, 0, 58));
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont1, x - 1, 30 + plusy, Color.FromArgb(0, 0, 0, 58));
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont1, x - 0, 29 + plusy, Color.FromArgb(0, 0, 0, 58));
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont1, x, 30 + plusy, Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
      }
      else
      {
        if (mode != 2)
          return;
        plusy += 40;
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont2, x + 1, 60 + plusy, Color.FromArgb(0, 0, 0, 65));
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont2, x, 61 + plusy, Color.FromArgb(0, 0, 0, 65));
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont2, x - 1, 60 + plusy, Color.FromArgb(0, 0, 0, 65));
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont2, x, 59 + plusy, Color.FromArgb(0, 0, 0, 65));
        DrawMod.DrawTextColouredMarcCenter(ref g, s, this.Game.introFont2, x, 60 + plusy, Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
      }
    }

    pub ScreenReturnClass HandleTimer()
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      this.Game.FormRef.Timer1.Interval = 1;
      DateTime now = DateAndTime.Now;
      TimeSpan timeSpan1 = now.Subtract(this.lasttime);
      if (timeSpan1.Milliseconds + timeSpan1.Seconds * 1000 > this.pagespeed)
      {
        if (this.pagenr >= this.lastpagenr)
        {
          screenReturnClass.NewScreen = 12;
          this.unloadAnyStuff();
        }
        else
        {
          this += 1.pagenr;
          this.lastfadetime = DateAndTime.Now;
          this.loadPageStuff(this.pagenr);
        }
        this.lasttime = DateAndTime.Now;
        screenReturnClass.flag = true;
      }
      TimeSpan timeSpan2 = now.Subtract(this.lastfadetime);
      if (timeSpan2.Milliseconds + timeSpan2.Seconds * 1000 > this.fadespeed)
      {
        this.lastfadetime = DateAndTime.Now;
        this += 1.fadenr;
        screenReturnClass.flag = true;
      }
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleMouseClick(int x, int y, int b)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      screenReturnClass.NewScreen = 12;
      screenReturnClass.flag = true;
      this.unloadAnyStuff();
      return screenReturnClass;
    }

    pub ScreenReturnClass HandleKeyPress(int nr)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          screenReturnClass.NewScreen = 12;
          screenReturnClass.flag = true;
          this.unloadAnyStuff();
          return screenReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return screenReturnClass;
    }

    pub fn unloadAnyStuff()
    {
      if (!Information.IsNothing( this.bmp))
      {
        this.bmp.Dispose();
        this.bmp = (Bitmap) null;
      }
      if (Information.IsNothing( this.lastbmp))
        return;
      this.lastbmp.Dispose();
      this.lastbmp = (Bitmap) null;
    }

    pub fn loadPageStuff(int nr)
    {
      if (nr == 1)
        this.loadSpecificBmp(this.Game.AppPath + "intrographics/se1.jpg");
      if (nr == 4)
        this.loadSpecificBmp(this.Game.AppPath + "intrographics/se2.jpg");
      if (nr == 7)
        this.loadSpecificBmp(this.Game.AppPath + "intrographics/se3.jpg");
      if (nr == 10)
        this.loadSpecificBmp(this.Game.AppPath + "intrographics/se4.jpg");
      if (nr == 13)
        this.loadSpecificBmp(this.Game.AppPath + "intrographics/se5.jpg");
      if (nr != 16)
        return;
      this.loadSpecificBmp(this.Game.AppPath + "intrographics/se6.jpg");
    }

    pub fn loadSpecificBmp(s: String)
    {
      if (!Information.IsNothing( this.lastbmp))
      {
        this.lastbmp.Dispose();
        this.lastbmp = (Bitmap) null;
      }
      if (!Information.IsNothing( this.bmp))
      {
        this.lastbmp = (Bitmap) this.bmp.Clone();
        this.bmp.Dispose();
        this.bmp = (Bitmap) null;
      }
      FileStream fileStream = new FileStream(s, FileMode.Open, FileAccess.Read);
      bitmap1: Bitmap = (Bitmap) Image.FromStream((Stream) fileStream);
      bitmap2: Bitmap = new Bitmap(bitmap1.Width, bitmap1.Height, PixelFormat.Format32bppPArgb);
      Graphics graphics = Graphics.FromImage((Image) bitmap2);
      graphics.DrawImage((Image) bitmap1, Rectangle::new(0, 0, bitmap1.Width, bitmap1.Height));
      graphics.Dispose();
      bitmap2.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      fileStream.Close();
      fileStream.Dispose();
      bitmap1.Dispose();
      this.bmp = bitmap2;
      this.fadenr = 1;
    }
  }
}
