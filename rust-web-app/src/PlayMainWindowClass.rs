// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PlayMainWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class PlayMainWindowClass : WindowClass
  {
     HexInfoId: i32;
     hexinfoid2: i32;
     MiniMapId: i32;
     minwidth: i32;
     detailnr: i32;
     MapId: i32;
     ListClass MapListObj;
     b1id: i32;
     b2id: i32;
     view1id: i32;
     view2id: i32;
     view3id: i32;

    pub PlayMainWindowClass( tGame: GameClass, let mut tminwidth: i32 =  0)
      : base( tGame, 220, tGame.ScreenHeight - (270 + tminwidth), 8)
    {
      this.minwidth = tminwidth;
      this.detailnr = this.game.EditObj.MapSelected;
      this.MakeShit();
      this.mainframe = false;
    }

    pub fn DoRefresh()
    {
      this.detailnr = this.game.EditObj.MapSelected;
      this.MakeShit();
    }

    pub fn PopUpRefresh() => this.MakeShit();

    pub WindowDescription: String(x: i32, y: i32)
    {
      if (this.game.SelectX < 0 || this.game.Data.Turn == -1)
        return "";
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
          return this.MouseText[index];
      }
      return "";
    }

    pub fn MakeShit()
    {
      SizeF sizeF = SizeF::new();
      bool Attack;
      if (this.game.EditObj.OrderType == 14)
        Attack = true;
      if (this.game.EditObj.OrderType == 15)
        Attack = true;
      if (this.game.EditObj.OrderType == 2)
        Attack = true;
      if (this.game.EditObj.OrderType == 12)
        Attack = true;
      if (this.game.EditObj.OrderType == 11)
        Attack = true;
      if (this.game.EditObj.OrderType == 13)
        Attack = true;
      if (this.MapId > 0)
        this.RemoveSubPart(this.MapId);
      if (this.HexInfoId > 0)
        this.RemoveSubPart(this.HexInfoId);
      if (this.hexinfoid2 > 0)
        this.RemoveSubPart(this.hexinfoid2);
      if (this.b1id > 0)
      {
        this.RemoveSubPart(this.b1id);
        this.b1id = 0;
      }
      if (this.b2id > 0)
      {
        this.RemoveSubPart(this.b2id);
        this.b2id = 0;
      }
      this.NewBackGroundAndClearAll(220, this.game.ScreenHeight - (270 + this.minwidth), -1);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      let mut ty: i32 =  70;
      let mut h1: i32 =  this.game.ScreenHeight - (270 + this.minwidth);
      this.ClearMouse();
      if (this.game.Data.Round == 0)
        this.game.Data.Turn = -1;
      str: String;
      if (this.game.Data.Turn > -1)
      {
        str = this.game.Data.RegimeObj[this.game.Data.Turn].Name;
      }
      else
      {
        str = "Editor".to_owned();
        DrawMod.DrawTextVic2( graphics, "Editor", this.game.VicFont1, 10, 1, this.game.VicColor2, this.game.VicColor1Shade);
      }
      if (this.game.Data.Round > 0)
      {
        DrawMod.DrawBlock( graphics, 0, 0, 5, h1,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
        DrawMod.DrawBlock( graphics, 5, 0, 220, 34,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
        DrawMod.DrawBlock( graphics, 5, ty + 114, 220, 34,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
        let mut h2: i32 =  h1 - (ty + 426);
        DrawMod.DrawBlock( graphics, 5, ty + 426, 220, h2,  this.game.VicColor4.R,  this.game.VicColor4.G,  this.game.VicColor4.B,  this.game.VicColor4.A);
        DrawMod.drawLine( graphics, 0, 34, 0, this.game.ScreenHeight - 265,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( graphics, 5, 34, 218, 34,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( graphics, 4, 34, 4, ty + 114, 50, 50, 50,  byte.MaxValue);
        DrawMod.drawLine( graphics, 5, 34, 5, ty + 114,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( graphics, 5, ty + 114, 218, ty + 114,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( graphics, 5, ty + 148, 218, ty + 148,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( graphics, 5, ty + 426, 218, ty + 426,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
        DrawMod.drawLine( graphics, 4, ty + 148, 4, ty + 426, 50, 50, 50,  byte.MaxValue);
        DrawMod.drawLine( graphics, 5, ty + 148, 5, ty + 426,  this.game.VicColor6.R,  this.game.VicColor6.G,  this.game.VicColor6.B,  this.game.VicColor6.A);
      }
      let mut num1: i32 =  ty + 130;
      if (this.game.Data.Round == 0)
      {
        ty = -80;
        num1 = 120;
      }
      if (this.view1id > 0)
      {
        this.RemoveSubPart(this.view1id);
        this.view1id = 0;
      }
      if (this.view2id > 0)
      {
        this.RemoveSubPart(this.view2id);
        this.view2id = 0;
      }
      if (this.view3id > 0)
      {
        this.RemoveSubPart(this.view3id);
        this.view3id = 0;
      }
      let mut num2: i32 =  190;
      if (this.game.Data.Round == 0)
        num2 = 35;
      if (this.game.Data.Round == 0)
      {
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Hex", 70, "Click here to view info on the hex",  this.OwnBitmap, 5, num2, tred: (this.game.EditObj.SetViewMode == 0), theight: 26);
        this.view1id = this.AddSubPart( tsubpart1, 5, num2, 70, 26, 1);
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Loc", 70, "Click here to view info on the location. If no location is present hex info is shown.",  this.OwnBitmap, 75, num2, tred: (this.game.EditObj.SetViewMode == 2), theight: 26);
        this.view2id = this.AddSubPart( tsubpart2, 75, num2, 70, 26, 1);
        if (this.game.ScreenHeight <= 928)
        {
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Map", 70, "Click here to view the minimap.",  this.OwnBitmap, 145, num2, tred: (this.game.EditObj.SetViewMode == 1), theight: 26);
          this.view3id = this.AddSubPart( tsubpart3, 145, num2, 70, 26, 1);
        }
      }
      else
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Hex", 70, "Click here to view info on the hex",  this.OwnBitmap, 40, num2, tred: (this.game.EditObj.SetViewMode == 0), theight: 26);
        this.view1id = this.AddSubPart( tsubpart4, 40, num2, 70, 26, 1);
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Loc", 70, "Click here to view info on the location. If no location is present hex info is shown.",  this.OwnBitmap, 120, num2, tred: (this.game.EditObj.SetViewMode == 2), theight: 26);
        this.view2id = this.AddSubPart( tsubpart5, 120, num2, 70, 26, 1);
      }
      if (this.game.Data.Round > 0)
      {
        let mut hqSpriteNr: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].HQSpriteNr;
        str = this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.Turn].People].Name;
        Rectangle paintedPartRect = DrawMod.GetPaintedPartRect(BitmapStore.GetBitmap(hqSpriteNr, 1));
         let mut local1: &Graphics = &graphics;
        bitmap1: Bitmap = BitmapStore.GetBitmap(hqSpriteNr, 1);
         let mut local2: &Bitmap = &bitmap1;
        let mut srcrect: &Rectangle = &paintedPartRect
        Rectangle rectangle = Rectangle::new(89, 12, 38, 28);
        let mut destrect: &Rectangle = &rectangle
        DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
        rectangle = Rectangle::new(80, 9, 53, 36);
        let mut trect: &Rectangle = &rectangle
        this.AddMouse( trect, "", "The current turn is to " + this.game.Data.RegimeObj[this.game.Data.Turn].Name + ". Click to change the of: Color the regime.", 1);
         let mut local3: &Graphics = &graphics;
        bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.ORNAMENT1);
         let mut local4: &Bitmap = &bitmap2;
        DrawMod.DrawSimple( local3,  local4, 10, 5);
        if (this.MiniMapId > 0)
          this.RemoveSubPart(this.MiniMapId);
        let mut tsubpart: SubPartClass =  new MiniMapPartClass(this.game, tx: 198, ty: 133);
        this.MiniMapId = this.AddSubPart( tsubpart, 11, 45, 198, 133, 0);
        DrawMod.DrawRectangle( graphics, 10, 44, 200, 135,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
      }
      else
        str = "Editor".to_owned();
      if (this.game.SelectX == -1)
      {
        this.game.SelectX = 0;
        this.game.SelectY = 0;
      }
      if (this.game.EditObj.Layout == 0 | this.game.Data.Round == 0)
      {
        if (this.game.EditObj.SetViewMode == 0 | this.game.EditObj.SetViewMode == 2 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location == -1 | this.game.EditObj.SetViewMode == 2 & Attack)
          this.MakeShit0(graphics, ty, Attack);
        else if (this.game.EditObj.SetViewMode == 1)
          this.MakeShit1(graphics, ty);
        else if (this.game.EditObj.SetViewMode == 2)
          this.MakeShit2(graphics, ty);
      }
      else if (this.game.EditObj.Layout != 1)
        ;
      if (Information.IsNothing( graphics))
        return;
      graphics.Dispose();
      graphics = (Graphics) null;
    }

    pub fn MakeShit0(Graphics g, ty: i32, bool Attack)
    {
      if (this.game.SelectX == -1 | this.game.SelectY == -1)
        return;
      let mut regime: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
      let mut location1: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      str1: String = this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].Name;
      if (location1 > -1)
        str1 = str1 + ", " + this.game.Data.LocTypeObj[this.game.Data.LocObj[location1].Type].Name;
      str2: String;
      num1: i32;
      if (this.game.Data.Round > 0 && this.game.Data.ShrowdOn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_SeeNow(this.game.Data.Turn) < 1)
      {
        str2 = "Shrouded (" + Conversion.Str( this.game.SelectX) + "," + Conversion.Str( this.game.SelectY) + ")";
        str1 = "Unkown type";
        num1 = 1;
      }
      let mut num2: i32 =  0;
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn)
        num2 = 1;
      if (this.game.Data.Round != 0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].MaxRecon > 0)
        num2 = 1;
      if (!this.game.Data.FOWOn)
        num2 = 1;
      if (this.game.Data.Round == 0)
        num2 = 1;
      str3: String;
      if (num1 == 0)
      {
        str3 = "";
        if (Information.IsNothing( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Name))
          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Name = "";
        if (location1 > -1)
          str2 = this.game.Data.LocObj[location1].Name + " " + "(" + Strings.Trim(Conversion.Str( this.game.SelectX)) + ", " + Strings.Trim(Conversion.Str( this.game.SelectY)) + ")";
        else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Name.Length > 0)
          str2 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Name + " " + "(" + Strings.Trim(Conversion.Str( this.game.SelectX)) + ", " + Strings.Trim(Conversion.Str( this.game.SelectY)) + ")";
        else
          str2 = "(" + Strings.Trim(Conversion.Str( this.game.SelectX)) + ", " + Strings.Trim(Conversion.Str( this.game.SelectY)) + ")";
      }
      if ( this.game.Data.RuleVar[900] == 1.0)
        str2 = str2 + ", rec=" + Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].MaxRecon);
      ty -= 5;
      Rectangle rectangle;
      Rectangle trect;
      if (num1 == 0)
      {
        let mut index: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
        if (this.game.Data.Round > 0 && this.game.Data.ShrowdOn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_SeeNow(this.game.Data.Turn) < 1)
          index = -2;
        if (index > -1)
        {
          let mut hqSpriteNr: i32 =  this.game.Data.RegimeObj[index].HQSpriteNr;
           let mut local1: &Graphics = &g;
          Rectangle rect1 = Rectangle::new(10, ty + 161, 200, 14);
          rectangle = Rectangle::new(10, ty + 175, 200, 26);
          let mut rect2: &Rectangle = &rectangle
          txt2: String = "        " + str2;
          DrawMod.MakeFullBoxVic2( local1, rect1, "SELECTED HEX", rect2, txt2);
          Rectangle paintedPartRect = DrawMod.GetPaintedPartRect(BitmapStore.GetBitmap(hqSpriteNr));
           let mut local2: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(hqSpriteNr);
           let mut local3: &Bitmap = &bitmap;
          let mut srcrect: &Rectangle = &paintedPartRect
          rectangle = Rectangle::new(15, ty + 179, 26, 20);
          let mut destrect: &Rectangle = &rectangle
          DrawMod.DrawSimplePart2( local2,  local3, srcrect, destrect);
        }
        else
        {
           let mut local: &Graphics = &g;
          rectangle = Rectangle::new(10, ty + 161, 200, 14);
          let mut rect1: &Rectangle = &rectangle
          trect = Rectangle::new(10, ty + 175, 200, 26);
          let mut rect2: &Rectangle = &trect
          txt2: String = str2;
          DrawMod.MakeFullBoxVic2( local, rect1, "SELECTED HEX", rect2, txt2);
        }
      }
      else
      {
         let mut local: &Graphics = &g;
        rectangle = Rectangle::new(10, ty + 161, 200, 14);
        let mut rect1: &Rectangle = &rectangle
        trect = Rectangle::new(10, ty + 175, 200, 26);
        let mut rect2: &Rectangle = &trect
        txt2: String = str2;
        DrawMod.MakeFullBoxVic2( local, rect1, "SELECTED HEX", rect2, txt2);
      }
      rectangle = Rectangle::new(10, ty + 161, 200, 40);
      trect = rectangle;
      this.AddMouse( trect, "", "The hex owners flag, coordinate and if applicable the name of the hex. If location is present you can rename it by clicking.", 2);
      ty += 5;
      str4: String = str1;
      landscapeType: i32;
      spriteNr: i32;
      SubPartClass tsubpart;
      if (num1 == 0)
      {
        landscapeType = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType;
        spriteNr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpriteNr;
        location1 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
        if (landscapeType > -1 & spriteNr > -1)
        {
          tsubpart =  new ATHexSubPartClass(this.game.SelectX, this.game.SelectY, this.game, true);
          this.HexInfoId = this.AddSubPart( tsubpart, 10, ty + 203, 200, 82, 0);
        }
      }
      if (Attack & this.game.Data.Round > 0)
      {
        let mut num3: i32 =  0;
        name: String = this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].LandscapeType].Name;
        str5: String;
        if (location1 > -1)
          str5 = name + ", " + this.game.Data.LocTypeObj[this.game.Data.LocObj[location1].Type].Name;
        if (this.game.Data.Round > 0 && this.game.Data.ShrowdOn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].get_SeeNow(this.game.Data.Turn) < 1)
        {
          str4 = "Shrouded (" + Conversion.Str( this.game.EditObj.TargetX) + "," + Conversion.Str( this.game.EditObj.TargetY) + ")";
          str5 = "Unkown type";
          num3 = 1;
        }
        if (num3 == 0)
        {
          str3 = "";
          let mut location2: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].Location;
          if (Information.IsNothing( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].Name))
            this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].Name = "";
          if (location2 > -1)
            str4 = this.game.Data.LocObj[location2].Name + " " + "(" + Strings.Trim(Conversion.Str( this.game.EditObj.TargetX)) + ", " + Strings.Trim(Conversion.Str( this.game.EditObj.TargetY)) + ")";
          else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].Name.Length > 0)
            str4 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.TargetX, this.game.EditObj.TargetY].Name + " " + "(" + Strings.Trim(Conversion.Str( this.game.EditObj.TargetX)) + ", " + Strings.Trim(Conversion.Str( this.game.EditObj.TargetY)) + ")";
          else
            str4 = Strings.Trim(Conversion.Str( this.game.EditObj.TargetX)) + ", " + Strings.Trim(Conversion.Str( this.game.EditObj.TargetY));
        }
         let mut local: &Graphics = &g;
        rectangle = Rectangle::new(10, ty + 290, 200, 14);
        let mut rect1: &Rectangle = &rectangle
        trect = Rectangle::new(10, ty + 304, 200, 23);
        let mut rect2: &Rectangle = &trect
        txt2: String = str4;
        DrawMod.MakeFullBoxVic2( local, rect1, "TARGET HEX", rect2, txt2);
        rectangle = Rectangle::new(10, ty + 290, 200, 40);
        trect = rectangle;
        this.AddMouse( trect, "", "The hex you are in the process of attacking. Shown below are units selected to participate in attack as well as known defenders.");
        if (!(landscapeType > -1 & spriteNr > -1))
          return;
        tsubpart =  new ATHexSubPartClass(this.game.SelectX, this.game.SelectY, this.game);
        this.hexinfoid2 = this.AddSubPart( tsubpart, 10, ty + 332, 200, 82, 0);
      }
      else
      {
        if (num1 == 0)
        {
           let mut local4: &Graphics = &g;
          rectangle = Rectangle::new(10, ty + 290, 170, 14);
          let mut rect1_1: &Rectangle = &rectangle
          trect = Rectangle::new(10, ty + 304, 170, 23);
          let mut rect2_1: &Rectangle = &trect
          txt2_1: String = str1;
          DrawMod.MakeFullBoxVic2( local4, rect1_1, "LANDSCAPE TYPE", rect2_1, txt2_1);
          ty -= 4;
          str6: String = Strings.Trim(Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].VP));
           let mut local5: &Graphics = &g;
          rectangle = Rectangle::new(10, ty + 335, 35, 14);
          let mut rect1_2: &Rectangle = &rectangle
          trect = Rectangle::new(10, ty + 349, 35, 23);
          let mut rect2_2: &Rectangle = &trect
          txt2_2: String = str6;
          DrawMod.MakeFullBoxVic2( local5, rect1_2, "VP", rect2_2, txt2_2);
          rectangle = Rectangle::new(10, ty + 335, 35, 40);
          trect = rectangle;
          this.AddMouse( trect, "", "Victory Points");
          if (this.game.Data.Turn > -1)
          {
            str7: String = Strings.Trim(Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].MaxRecon));
            if ( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].MaxRecon <  this.game.Data.RuleVar[55])
            {
               let mut local6: &Graphics = &g;
              rectangle = Rectangle::new(65, ty + 335, 35, 14);
              let mut rect1_3: &Rectangle = &rectangle
              trect = Rectangle::new(65, ty + 349, 35, 23);
              let mut rect2_3: &Rectangle = &trect
              txt2_3: String = str7;
              DrawMod.MakeFullBoxVic2( local6, rect1_3, "REC", rect2_3, txt2_3);
            }
            else if ( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].MaxRecon <  this.game.Data.RuleVar[56])
            {
               let mut local7: &Graphics = &g;
              rectangle = Rectangle::new(65, ty + 335, 35, 14);
              let mut rect1_4: &Rectangle = &rectangle
              trect = Rectangle::new(65, ty + 349, 35, 23);
              let mut rect2_4: &Rectangle = &trect
              txt2_4: String = str7;
              DrawMod.MakeFullBoxVic2( local7, rect1_4, "REC", rect2_4, txt2_4);
            }
            else
            {
               let mut local8: &Graphics = &g;
              rectangle = Rectangle::new(65, ty + 335, 35, 14);
              let mut rect1_5: &Rectangle = &rectangle
              trect = Rectangle::new(65, ty + 349, 35, 23);
              let mut rect2_5: &Rectangle = &trect
              txt2_5: String = str7;
              DrawMod.MakeFullBoxVic2( local8, rect1_5, "REC", rect2_5, txt2_5);
            }
          }
          rectangle = Rectangle::new(65, ty + 335, 35, 40);
          trect = rectangle;
          this.AddMouse( trect, "", "Recon Points");
          if (this.game.Data.Turn > -1)
          {
            str8: String = Strings.Trim(Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_ZocPts(this.game.Data.Turn)));
             let mut local9: &Graphics = &g;
            rectangle = Rectangle::new(120, ty + 335, 35, 14);
            let mut rect1_6: &Rectangle = &rectangle
            trect = Rectangle::new(120, ty + 349, 35, 23);
            let mut rect2_6: &Rectangle = &trect
            txt2_6: String = str8;
            DrawMod.MakeFullBoxVic2( local9, rect1_6, "ZOC", rect2_6, txt2_6);
          }
          rectangle = Rectangle::new(120, ty + 335, 35, 40);
          trect = rectangle;
          this.AddMouse( trect, "", "Zone of Control Points");
          str9: String;
          if (num2 == 1)
          {
            str9 = Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.GetHexStackPts(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected)));
            if (this.game.Data.FOWOn)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1 && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime, this.game.Data.Turn))
                str9 = "?";
              if (this.game.EditObj.UnitSelected > -1 && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn))
                str9 = "?";
            }
          }
          else
            str9 = "?";
           let mut local10: &Graphics = &g;
          rectangle = Rectangle::new(10, ty + 377, 35, 14);
          let mut rect1_7: &Rectangle = &rectangle
          trect = Rectangle::new(10, ty + 390, 35, 23);
          let mut rect2_7: &Rectangle = &trect
          txt2_7: String = str9;
          DrawMod.MakeFullBoxVic2( local10, rect1_7, "STK", rect2_7, txt2_7);
          rectangle = Rectangle::new(10, ty + 377, 35, 40);
          trect = rectangle;
          this.AddMouse( trect, "", "Stack Points");
          if (this.game.Data.Turn > -1)
          {
            let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_APPenalty(this.game.Data.Turn) + this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_BattlePenalty(this.game.Data.Turn);
            str10: String = Strings.Trim(Conversion.Str( Number));
            if (0 > Number)
              str10 = "-" + str10;
             let mut local11: &Graphics = &g;
            rectangle = Rectangle::new(65, ty + 377, 45, 14);
            let mut rect1_8: &Rectangle = &rectangle
            trect = Rectangle::new(65, ty + 390, 35, 23);
            let mut rect2_8: &Rectangle = &trect
            txt2_8: String = str10;
            DrawMod.MakeFullBoxVic2( local11, rect1_8, "+AP", rect2_8, txt2_8);
            rectangle = Rectangle::new(65, ty + 377, 45, 40);
            trect = rectangle;
            this.AddMouse( trect, "", "Action PoPenalty: i32 (for entering hex)");
            str11: String = Strings.Trim(Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_BattleStack(this.game.Data.Turn)));
             let mut local12: &Graphics = &g;
            rectangle = Rectangle::new(120, ty + 377, 45, 14);
            let mut rect1_9: &Rectangle = &rectangle
            trect = Rectangle::new(120, ty + 390, 35, 23);
            let mut rect2_9: &Rectangle = &trect
            txt2_9: String = str11;
            DrawMod.MakeFullBoxVic2( local12, rect1_9, "BSLND", rect2_9, txt2_9);
            rectangle = Rectangle::new(120, ty + 377, 45, 40);
            trect = rectangle;
            this.AddMouse( trect, "", "Battlestack points for regular land combat");
            str12: String = Strings.Trim(Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_BattleStackArt(this.game.Data.Turn)));
             let mut local13: &Graphics = &g;
            rectangle = Rectangle::new(175, ty + 335, 45, 14);
            let mut rect1_10: &Rectangle = &rectangle
            trect = Rectangle::new(175, ty + 348, 35, 23);
            let mut rect2_10: &Rectangle = &trect
            txt2_10: String = str12;
            DrawMod.MakeFullBoxVic2( local13, rect1_10, "BSART", rect2_10, txt2_10);
            rectangle = Rectangle::new(175, ty + 335, 45, 40);
            trect = rectangle;
            this.AddMouse( trect, "", "Battlestack points for artillery barrages");
            str13: String = Strings.Trim(Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_BattleStackAir(this.game.Data.Turn)));
             let mut local14: &Graphics = &g;
            rectangle = Rectangle::new(175, ty + 377, 45, 14);
            let mut rect1_11: &Rectangle = &rectangle
            trect = Rectangle::new(175, ty + 390, 35, 23);
            let mut rect2_11: &Rectangle = &trect
            txt2_11: String = str13;
            DrawMod.MakeFullBoxVic2( local14, rect1_11, "BSAIR", rect2_11, txt2_11);
            rectangle = Rectangle::new(175, ty + 377, 45, 40);
            trect = rectangle;
            this.AddMouse( trect, "", "Battlestack points for air to ground attacks");
          }
        }
        if (!(this.game.Data.Round == 0 & this.game.ScreenHeight > 928))
          return;
        if (this.MiniMapId > 0)
          this.RemoveSubPart(this.MiniMapId);
        tsubpart =  new MiniMapPartClass(this.game, tx: 198, ty: 133);
        this.MiniMapId = this.AddSubPart( tsubpart, 11, DrawMod.TGame.ScreenHeight - 370 - 160, 198, 133, 0);
        DrawMod.DrawRectangle( g, 10, ty + 161 - 1, 200, 135,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
      }
    }

    pub fn MakeShit1(Graphics g, ty: i32)
    {
      if (this.MiniMapId > 0)
        this.RemoveSubPart(this.MiniMapId);
      let mut tsubpart1: SubPartClass =  new MiniMapPartClass(this.game, tx: 198, ty: 133);
      this.MiniMapId = this.AddSubPart( tsubpart1, 11, ty + 161, 198, 133, 0);
      DrawMod.DrawRectangle( g, 10, ty + 161 - 1, 200, 135,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
      if (this.game.Data.MapCounter <= 0)
        return;
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      this.MapListObj = ListClass::new();
      let mut mapCounter: i32 =  this.game.Data.MapCounter;
      for (let mut tdata: i32 =  0; tdata <= mapCounter; tdata += 1)
      {
        let mut num3: i32 =  0;
        if (this.game.Data.Round == 0)
          num3 = 1;
        if (this.game.EditObj.OrderType < 1 & this.game.Data.MapObj[tdata].CanSee)
          num3 = 1;
        if (this.game.EditObj.OrderType < 1 & !this.game.Data.ShrowdOn)
          num3 = 1;
        if (this.game.EditObj.OrderType > 0 & this.game.Data.MapObj[tdata].TempCanSee)
          num3 = 1;
        if (num3 == 1)
        {
          num2 += 1;
          this.MapListObj.add(this.game.Data.MapObj[tdata].Name, tdata);
          if (this.detailnr == tdata)
            num1 = num2;
        }
      }
      ListClass mapListObj = this.MapListObj;
      let mut tlistselect: i32 =  num1;
      let mut game: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart2: SubPartClass =  new ListSubPartClass(mapListObj, 6, 180, tlistselect, game, tHeader: "Maps", tbackbitmap: ( local1), bbx: 10, bby: 355, overruleFont: ( local2));
      this.MapId = this.AddSubPart( tsubpart2, 10, 355, 120, 144, 0);
    }

    pub fn MakeShit2(Graphics g, ty: i32)
    {
      if (this.game.SelectX == -1 | this.game.SelectY == -1)
        return;
      let mut regime: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
      let mut location: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      if (this.game.Data.Round > 0 && this.game.Data.ShrowdOn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].get_SeeNow(this.game.Data.Turn) < 1)
      {
        bool Attack;
        if (this.game.EditObj.OrderType == 14)
          Attack = true;
        if (this.game.EditObj.OrderType == 15)
          Attack = true;
        if (this.game.EditObj.OrderType == 2)
          Attack = true;
        if (this.game.EditObj.OrderType == 12)
          Attack = true;
        if (this.game.EditObj.OrderType == 11)
          Attack = true;
        if (this.game.EditObj.OrderType == 13)
          Attack = true;
        this.MakeShit0(g, ty, Attack);
      }
      else
      {
        if (location > -1)
        {
          name: String = this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].Name;
           let mut local1: &Graphics = &g;
          Rectangle rectangle1 = Rectangle::new(10, ty + 161, 160, 14);
          let mut rect1_1: &Rectangle = &rectangle1
          Rectangle rectangle2 = Rectangle::new(10, ty + 175, 160, 23);
          let mut rect2_1: &Rectangle = &rectangle2
          txt2_1: String = name;
          DrawMod.MakeFullBoxVic2( local1, rect1_1, "LOCATION TYPE", rect2_1, txt2_1);
          let mut people: i32 =  this.game.Data.LocObj[location].People;
          let mut hq: i32 =  this.game.Data.LocObj[location].HQ;
          str1: String = Strings.UCase(Strings.Left(this.game.Data.PeopleObj[people].Name, 3));
           let mut local2: &Graphics = &g;
          rectangle2 = Rectangle::new(10, ty + 203, 35, 14);
          let mut rect1_2: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(10, ty + 217, 35, 23);
          let mut rect2_2: &Rectangle = &rectangle1
          txt2_2: String = str1;
          DrawMod.MakeFullBoxVic2( local2, rect1_2, "PPL", rect2_2, txt2_2);
          rectangle2 = Rectangle::new(10, ty + 203, 35, 40);
          let mut trect: &Rectangle = &rectangle2
          this.AddMouse( trect, "", "People that live/work in the location");
          if (this.game.Data.Turn > -1)
          {
            str2: String = this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].get_ReconPts(this.game.Data.Turn) <= 0 ? "? / " + Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].StructuralPts)) : Strings.Trim(Conversion.Str( this.game.Data.LocObj[location].StructuralPts)) + " / " + Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].StructuralPts));
             let mut local3: &Graphics = &g;
            rectangle2 = Rectangle::new(55, ty + 203, 95, 14);
            let mut rect1_3: &Rectangle = &rectangle2
            trect = Rectangle::new(55, ty + 217, 95, 23);
            let mut rect2_3: &Rectangle = &trect
            txt2_3: String = str2;
            DrawMod.MakeFullBoxVic2( local3, rect1_3, "STRUCTURAL", rect2_3, txt2_3);
            rectangle2 = Rectangle::new(55, ty + 203, 95, 40);
            trect = rectangle2;
            this.AddMouse( trect, "", "Structural points the location currently has / can maximum have");
          }
          str3: String = this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].AutoRecoverPts <= 0 ? "0" : "+" + Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].AutoRecoverPts));
           let mut local4: &Graphics = &g;
          rectangle2 = Rectangle::new(160, ty + 203, 55, 14);
          let mut rect1_4: &Rectangle = &rectangle2
          trect = Rectangle::new(160, ty + 217, 50, 23);
          let mut rect2_4: &Rectangle = &trect
          txt2_4: String = str3;
          DrawMod.MakeFullBoxVic2( local4, rect1_4, "AUTOREP", rect2_4, txt2_4);
          rectangle2 = Rectangle::new(160, ty + 203, 55, 40);
          trect = rectangle2;
          this.AddMouse( trect, "", "Number of autorepair points that are applied to the location each round");
          if (!this.game.Data.FOWOn | this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[location].X, this.game.Data.LocObj[location].Y].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
          {
            str4: String;
            if (this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].NoHQ)
            {
              str4 = "Needs no Hq";
              this.game.Data.LocObj[location].HQ = -1;
            }
            else
              str4 = hq <= -1 ? "No Hq" : this.game.Data.UnitObj[hq].Name;
             let mut local5: &Graphics = &g;
            rectangle2 = Rectangle::new(10, ty + 245, 200, 14);
            let mut rect1_5: &Rectangle = &rectangle2
            trect = Rectangle::new(10, ty + 259, 200, 23);
            let mut rect2_5: &Rectangle = &trect
            txt2_5: String = str4;
            DrawMod.MakeFullBoxVic2( local5, rect1_5, "HQ", rect2_5, txt2_5);
            rectangle2 = Rectangle::new(10, ty + 245, 200, 40);
            trect = rectangle2;
            this.AddMouse( trect, "", "The HQ that the location tries to deliver its production too");
            let mut prodslot: i32 =  0;
            do
            {
              float Number = 0.0f;
              if (this.game.Data.LocObj[location].Production[prodslot] > -1)
              {
                let mut index: i32 =  this.game.Data.LocObj[location].Production[prodslot];
                Left: String = Strings.Left(this.game.Data.ItemTypeObj[index].Name, 12);
                if (this.game.Data.ItemTypeObj[index].IsSupply)
                  Left = "Supplies".to_owned();
                if (this.game.Data.ItemTypeObj[index].IsResPt)
                  Left = "Political".to_owned();
                if (this.game.Data.ItemTypeObj[index].IsSFType > -1)
                  Left = Strings.Left(this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[index].IsSFType].Name, 12);
                Number =  this.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, location, true, false, true);
                if (Operators.CompareString(Left, "Supplies", false) == 0)
                  Number = Conversion.Int(Number);
                str4 = Strings.Trim(Conversion.Str( Number)) + "x " + Left;
              }
              if ( Number <= 0.0)
                str4 = "";
              if (prodslot == 0)
              {
                 let mut local6: &Graphics = &g;
                rectangle2 = Rectangle::new(10, ty + 287, 200, 14);
                let mut rect1_6: &Rectangle = &rectangle2
                trect = Rectangle::new(10, ty + 301, 200, 23);
                let mut rect2_6: &Rectangle = &trect
                txt2_6: String = str4;
                DrawMod.MakeFullBoxVic2( local6, rect1_6, "PRODUCTION SLOTS", rect2_6, txt2_6);
              }
              rectangle2 = Rectangle::new(10, ty + 301, 200, 23);
              trect = rectangle2;
              this.AddMouse( trect, "", "Production slot #1");
              Rectangle rectangle3;
              if (prodslot == 2)
              {
                 let mut local7: &Graphics = &g;
                let mut rect1_7: &Rectangle = &rectangle3
                rectangle2 = Rectangle::new(10, ty + 357, 200, 23);
                let mut rect2_7: &Rectangle = &rectangle2
                txt2_7: String = str4;
                DrawMod.MakeFullBoxVic2( local7, rect1_7, "", rect2_7, txt2_7);
              }
              rectangle2 = Rectangle::new(10, ty + 357, 200, 23);
              trect = rectangle2;
              this.AddMouse( trect, "", "Production slot #3");
              if (prodslot == 1)
              {
                 let mut local8: &Graphics = &g;
                let mut rect1_8: &Rectangle = &rectangle3
                rectangle2 = Rectangle::new(10, ty + 329, 200, 23);
                let mut rect2_8: &Rectangle = &rectangle2
                txt2_8: String = str4;
                DrawMod.MakeFullBoxVic2( local8, rect1_8, "", rect2_8, txt2_8);
              }
              rectangle2 = Rectangle::new(10, ty + 329, 200, 23);
              trect = rectangle2;
              this.AddMouse( trect, "", "Production slot #2");
              if (prodslot == 3)
              {
                 let mut local9: &Graphics = &g;
                let mut rect1_9: &Rectangle = &rectangle3
                rectangle2 = Rectangle::new(10, ty + 385, 200, 23);
                let mut rect2_9: &Rectangle = &rectangle2
                txt2_9: String = str4;
                DrawMod.MakeFullBoxVic2( local9, rect1_9, "", rect2_9, txt2_9);
              }
              rectangle2 = Rectangle::new(10, ty + 385, 200, 23);
              trect = rectangle2;
              this.AddMouse( trect, "", "Production slot #4");
              prodslot += 1;
            }
            while (prodslot <= 3);
          }
        }
        if (!(this.game.Data.Round == 0 & this.game.ScreenHeight > 928))
          return;
        if (this.MiniMapId > 0)
          this.RemoveSubPart(this.MiniMapId);
        let mut tsubpart: SubPartClass =  new MiniMapPartClass(this.game, tx: 198, ty: 133);
        this.MiniMapId = this.AddSubPart( tsubpart, 11, DrawMod.TGame.ScreenHeight - 370 - 160, 198, 133, 0);
        DrawMod.DrawRectangle( g, 10, ty + 161 - 1, 200, 135,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (this.MouseData[index] > 0 && x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] == 1)
          {
            ColorDialog colorDialog = ColorDialog::new();
            colorDialog.Color = Color.FromArgb( byte.MaxValue, this.game.Data.RegimeObj[this.game.Data.Turn].Red, this.game.Data.RegimeObj[this.game.Data.Turn].Green, this.game.Data.RegimeObj[this.game.Data.Turn].Blue);
            if (colorDialog.ShowDialog() == DialogResult.OK)
            {
              this.game.Data.RegimeObj[this.game.Data.Turn].Blue =  colorDialog.Color.B;
              this.game.Data.RegimeObj[this.game.Data.Turn].Green =  colorDialog.Color.G;
              this.game.Data.RegimeObj[this.game.Data.Turn].Red =  colorDialog.Color.R;
            }
            this.game.Data.RegimeObj[this.game.Data.Turn].DoTempCounter();
            this.game.Data.RegimeObj[this.game.Data.Turn].DoTempCounterBig();
            this.game.Data.RegimeObj[this.game.Data.Turn].DoTempCounterSmall();
            this.MakeShit();
            windowReturnClass.AddCommand(4, 12);
            windowReturnClass.AddCommand(4, 20);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (this.MouseData[index] == 2 && this.game.SelectX > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn)
          {
            let mut location: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
            if (location > -1)
            {
              str: String = Interaction.InputBox("Give New Name for Location", "Rename", this.game.Data.LocObj[location].Name);
              if ( this.game.Data.RuleVar[419] > 0.0 & Operators.CompareString(Strings.Trim(str), "", false) != 0)
              {
                this.game.HandyFunctionsObj.MakeSpecificAutoLabels(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected,  Math.Round( this.game.Data.RuleVar[419]), Strings.Space(Strings.Len(this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Name)), true);
                this.game.HandyFunctionsObj.MakeSpecificAutoLabels(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected,  Math.Round( this.game.Data.RuleVar[419]), str, true);
              }
              if (Operators.CompareString(Strings.Trim(str), "", false) != 0)
                this.game.Data.LocObj[location].Name = str;
              this.MakeShit();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
      }
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.view1id)
            {
              this.game.EditObj.SetViewMode = 0;
              if (this.MiniMapId > 0)
              {
                this.RemoveSubPart(this.MiniMapId);
                this.MiniMapId = 0;
              }
              this.MakeShit();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.view2id)
            {
              this.game.EditObj.SetViewMode = 2;
              if (this.MiniMapId > 0)
              {
                this.RemoveSubPart(this.MiniMapId);
                this.MiniMapId = 0;
              }
              this.MakeShit();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.view3id)
            {
              this.game.EditObj.SetViewMode = 1;
              if (this.MiniMapId > 0)
              {
                this.RemoveSubPart(this.MiniMapId);
                this.MiniMapId = 0;
              }
              this.MakeShit();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b1id)
            {
              this.game.EditObj.LocTypeSelected = -1;
              this.game.EditObj.PopupValue = 12;
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.AddCommand(5, 10);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.b2id)
            {
              this.game.EditObj.PeopleSelected = this.game.Data.RegimeObj[this.game.Data.Turn].People;
              this.game.EditObj.PopupValue = 13;
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.AddCommand(5, 10);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            selectX: i32;
            selectY: i32;
            if (num1 == this.MiniMapId)
            {
              selectX = this.game.SelectX;
              selectY = this.game.SelectY;
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.MakeShit();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
            }
            else
            {
              if (num1 == this.MapId)
              {
                let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                if (num2 > -1)
                {
                  this.detailnr = num2;
                  this.game.EditObj.MapSelected = num2;
                  this.game.CornerX = 0;
                  this.game.CornerY = 0;
                  this.game.SelectX = 0;
                  this.game.SelectY = 0;
                  this.game.EditObj.UnitSelected = -1;
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 200, 150, false);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 20);
                  windowReturnClass.AddCommand(4, 18);
                  windowReturnClass.SetFlag(true);
                }
                return windowReturnClass;
              }
              if (num1 == this.HexInfoId || num1 == this.hexinfoid2)
              {
                let mut num3: i32 =  0;
                selectX = this.game.SelectX;
                selectY = this.game.SelectY;
                let mut index2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                if (index2 > -1)
                {
                  this.game.EditObj.UnitSelected = index2;
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  if (this.game.SelectX != this.game.Data.UnitObj[index2].X | this.game.SelectY != this.game.Data.UnitObj[index2].Y)
                    num3 = 1;
                  this.game.SelectX = this.game.Data.UnitObj[index2].X;
                  this.game.SelectY = this.game.Data.UnitObj[index2].Y;
                  this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                  windowReturnClass.AddCommand(4, 12);
                  if (this.game.EditObj.OrderType == 9)
                  {
                    if (this.game.EditObj.OrderTarget != this.game.EditObj.UnitSelected && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit && this.game.Data.Round > 0 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                    {
                      this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                      windowReturnClass.AddCommand(4, 30);
                      windowReturnClass.AddCommand(4, 18);
                    }
                  }
                  else if (this.game.EditObj.OrderType == 18)
                  {
                    if (this.game.EditObj.OrderTarget != -1 && this.game.EditObj.OrderTarget != this.game.EditObj.UnitSelected && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit && this.game.Data.Round > 0 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                    {
                      this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                      windowReturnClass.AddCommand(4, 35);
                    }
                  }
                  else
                    windowReturnClass.AddCommand(4, 20);
                  windowReturnClass.AddCommand(4, 44);
                  if (num3 == 1)
                    this.MakeShit();
                  else
                    this.PaintSpecific(this.SubPartID[index1]);
                  windowReturnClass.SetFlag(true);
                }
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn && x > 0 & x < 150 & y > 2 & y < 18)
                {
                  tempstr: String = Interaction.InputBox("Give New Name for Location", "Rename", this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Name);
                  if ( this.game.Data.RuleVar[419] > 0.0)
                  {
                    this.game.HandyFunctionsObj.MakeSpecificAutoLabels(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected,  Math.Round( this.game.Data.RuleVar[419]), Strings.Space(Strings.Len(this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Name)));
                    this.game.HandyFunctionsObj.MakeSpecificAutoLabels(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected,  Math.Round( this.game.Data.RuleVar[419]), tempstr);
                  }
                  this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Name = tempstr;
                  this.MakeShit();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
