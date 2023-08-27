// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PlaySecondMainWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class PlaySecondMainWindowClass : WindowClass
  {
     HexInfoId: i32;
     hexinfoid2: i32;
     MiniMapId: i32;
     minwidth: i32;
     detailnr: i32;
     MapId: i32;
     ListClass MapListObj;
     b1id: i32;

    pub PlaySecondMainWindowClass( tGame: GameClass, let mut tminwidth: i32 = 0)
      : base( tGame, 220, tGame.ScreenHeight - (305 + tminwidth), 8)
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

    pub WindowDescription: String(x: i32, y: i32)
    {
      if (this.game.SelectX < 0 || this.game.Data.Turn == -1)
        return "";
      let mut mouseCounter: i32 = this.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
          return this.MouseText[index];
      }
      return "";
    }

    pub fn PopUpRefresh() => this.MakeShit();

    pub fn MakeShit()
    {
      SizeF sizeF = SizeF::new();
      bool flag;
      if (this.game.EditObj.OrderType == 14)
        flag = true;
      if (this.game.EditObj.OrderType == 15)
        flag = true;
      if (this.game.EditObj.OrderType == 2)
        flag = true;
      if (this.game.EditObj.OrderType == 12)
        flag = true;
      if (this.game.EditObj.OrderType == 11)
        flag = true;
      if (this.game.EditObj.OrderType == 13)
        flag = true;
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
      this.NewBackGroundAndClearAll(220, this.game.ScreenHeight - (305 + this.minwidth), -1);
      Graphics.FromImage((Image) this.OwnBitmap);
    }

    pub fn MakeShit0(Graphics g, ty: i32, bool Attack)
    {
    }

    pub fn MakeShit1(Graphics g, ty: i32)
    {
      Rectangle rect2;
      DrawMod.MakeFullBoxVic2( g, Rectangle::new(10, ty + 161, 200, 14), "MINIMAP", rect2, "");
      if (this.MiniMapId > 0)
        this.RemoveSubPart(this.MiniMapId);
      let mut tsubpart: SubPartClass =  new MiniMapPartClass(this.game, tx: 198, ty: 138);
      this.MiniMapId = this.AddSubPart( tsubpart, 11, ty + 176, 198, 138, 0);
      DrawMod.DrawRectangle( g, 10, ty + 175, 200, 140,  this.game.VicColor3.R,  this.game.VicColor3.G,  this.game.VicColor3.B,  this.game.VicColor3.A);
      if (this.game.Data.MapCounter <= 0)
        return;
      let mut num1: i32 = -1;
      let mut num2: i32 = -1;
      this.MapListObj = ListClass::new();
      let mut mapCounter: i32 = this.game.Data.MapCounter;
      for (let mut tdata: i32 = 0; tdata <= mapCounter; tdata += 1)
      {
        let mut num3: i32 = 0;
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
      let mut tlistselect: i32 = num1;
      let mut game: GameClass = this.game;
       local1: Bitmap =  this.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      tsubpart =  new ListSubPartClass(mapListObj, 6, 180, tlistselect, game, tHeader: "Maps", tbackbitmap: ( local1), bbx: 10, bby: 355, overruleFont: ( local2));
      this.MapId = this.AddSubPart( tsubpart, 10, 355, 120, 144, 0);
    }

    pub fn MakeShit2(Graphics g, ty: i32)
    {
      if (this.game.SelectX == -1 | this.game.SelectY == -1)
        return;
      let mut regime: i32 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
      let mut location: i32 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (location == -1)
      {
        str: String = "No location in hex";
         let mut local: &Graphics = &g;
        rectangle1 = Rectangle::new(10, ty + 161, 200, 14);
        let mut rect1: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(10, ty + 175, 200, 23);
        let mut rect2: &Rectangle = &rectangle2
        txt2: String = str;
        DrawMod.MakeFullBoxVic2( local, rect1, "LOCATION TYPE", rect2, txt2);
      }
      if (location <= -1)
        return;
      name: String = this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].Name;
       let mut local1: &Graphics = &g;
      rectangle2 = Rectangle::new(10, ty + 161, 165, 14);
      let mut rect1_1: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(10, ty + 175, 165, 23);
      let mut rect2_1: &Rectangle = &rectangle1
      txt2_1: String = name;
      DrawMod.MakeFullBoxVic2( local1, rect1_1, "LOCATION TYPE", rect2_1, txt2_1);
      let mut tsubpart: SubPartClass =  new TextButtonPartClass("?", 30, "Get more information on selected landscape and/or location type",  this.OwnBitmap, 180, ty + 175, theight: 25);
      this.b1id = this.AddSubPart( tsubpart, 180, ty + 175, 30, 25, 1);
      let mut people: i32 = this.game.Data.LocObj[location].People;
      let mut hq: i32 = this.game.Data.LocObj[location].HQ;
      str1: String = Strings.UCase(Strings.Left(this.game.Data.PeopleObj[people].Name, 3));
       let mut local2: &Graphics = &g;
      rectangle2 = Rectangle::new(10, ty + 203, 35, 14);
      let mut rect1_2: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(10, ty + 217, 35, 23);
      let mut rect2_2: &Rectangle = &rectangle1
      txt2_2: String = str1;
      DrawMod.MakeFullBoxVic2( local2, rect1_2, "PPL", rect2_2, txt2_2);
      str2: String = this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].get_ReconPts(this.game.Data.Turn) <= 0 ? "? / " + Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].StructuralPts)) : Strings.Trim(Conversion.Str( this.game.Data.LocObj[location].StructuralPts)) + " / " + Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].StructuralPts));
       let mut local3: &Graphics = &g;
      rectangle2 = Rectangle::new(55, ty + 203, 95, 14);
      let mut rect1_3: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(55, ty + 217, 95, 23);
      let mut rect2_3: &Rectangle = &rectangle1
      txt2_3: String = str2;
      DrawMod.MakeFullBoxVic2( local3, rect1_3, "STRUCTURAL", rect2_3, txt2_3);
      str3: String = this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].AutoRecoverPts <= 0 ? "0" : "+" + Strings.Trim(Conversion.Str( this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].AutoRecoverPts));
       let mut local4: &Graphics = &g;
      rectangle2 = Rectangle::new(160, ty + 203, 55, 14);
      let mut rect1_4: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(160, ty + 217, 50, 23);
      let mut rect2_4: &Rectangle = &rectangle1
      txt2_4: String = str3;
      DrawMod.MakeFullBoxVic2( local4, rect1_4, "AUTOREP", rect2_4, txt2_4);
      if (!(!this.game.Data.FOWOn | this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[location].X, this.game.Data.LocObj[location].Y].Regime == this.game.Data.Turn | this.game.Data.Round == 0))
        return;
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
      rectangle1 = Rectangle::new(10, ty + 259, 200, 23);
      let mut rect2_5: &Rectangle = &rectangle1
      txt2_5: String = str4;
      DrawMod.MakeFullBoxVic2( local5, rect1_5, "HQ", rect2_5, txt2_5);
      let mut index1: i32 = 0;
      do
      {
        float Number = 0.0f;
        if (this.game.Data.LocObj[location].Production[index1] > -1)
        {
          let mut index2: i32 = this.game.Data.LocObj[location].Production[index1];
          Left: String = Strings.Left(this.game.Data.ItemTypeObj[index2].Name, 12);
          if (this.game.Data.ItemTypeObj[index2].IsSupply)
            Left = "Supplies";
          if (this.game.Data.ItemTypeObj[index2].IsResPt)
            Left = "Political";
          if (this.game.Data.ItemTypeObj[index2].IsSFType > -1)
            Left = Strings.Left(this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[index2].IsSFType].Name, 12);
          Number = Conversions.ToSingle(Strings.Trim(Conversion.Str( this.game.Data.LocObj[location].TempProdPredict[index1])));
          if (Operators.CompareString(Left, "Supplies", false) == 0)
            Number = Conversion.Int(Number);
          str4 = Strings.Trim(Conversion.Str( Number)) + "x " + Left;
        }
        if ( Number <= 0.0)
          str4 = "";
        if (index1 == 0)
        {
           let mut local6: &Graphics = &g;
          rectangle2 = Rectangle::new(10, ty + 287, 200, 14);
          let mut rect1_6: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(10, ty + 301, 200, 23);
          let mut rect2_6: &Rectangle = &rectangle1
          txt2_6: String = str4;
          DrawMod.MakeFullBoxVic2( local6, rect1_6, "PRODUCTION SLOTS", rect2_6, txt2_6);
        }
        Rectangle rectangle3;
        if (index1 == 2)
        {
           let mut local7: &Graphics = &g;
          let mut rect1_7: &Rectangle = &rectangle3
          rectangle2 = Rectangle::new(10, ty + 357, 200, 23);
          let mut rect2_7: &Rectangle = &rectangle2
          txt2_7: String = str4;
          DrawMod.MakeFullBoxVic2( local7, rect1_7, "", rect2_7, txt2_7);
        }
        if (index1 == 1)
        {
           let mut local8: &Graphics = &g;
          let mut rect1_8: &Rectangle = &rectangle3
          rectangle2 = Rectangle::new(10, ty + 329, 200, 23);
          let mut rect2_8: &Rectangle = &rectangle2
          txt2_8: String = str4;
          DrawMod.MakeFullBoxVic2( local8, rect1_8, "", rect2_8, txt2_8);
        }
        if (index1 == 3)
        {
           let mut local9: &Graphics = &g;
          let mut rect1_9: &Rectangle = &rectangle3
          rectangle2 = Rectangle::new(10, ty + 385, 200, 23);
          let mut rect2_9: &Rectangle = &rectangle2
          txt2_9: String = str4;
          DrawMod.MakeFullBoxVic2( local9, rect1_9, "", rect2_9, txt2_9);
        }
        index1 += 1;
      }
      while (index1 <= 3);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = this.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (this.MouseData[index] > 0 && x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width)
        {
          let mut num: i32 = y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height ? 1 : 0;
        }
      }
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = this.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num: i32 = this.SubPartID[index];
            if (num == this.b1id)
            {
              this.game.EditObj.LocTypeSelected = -1;
              this.game.EditObj.PopupValue = 12;
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.AddCommand(5, 10);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.MiniMapId)
            {
              let mut selectX: i32 = this.game.SelectX;
              let mut selectY: i32 = this.game.SelectY;
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.MakeShit();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
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
