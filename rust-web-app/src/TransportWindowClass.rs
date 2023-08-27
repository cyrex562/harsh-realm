// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TransportWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TransportWindowClass : WindowClass
  {
     okid: i32;
     cancelid: i32;
     oktextid: i32;
     Pic1Id: i32;
     His: i32;
     Card: i32;
     Unr: i32;
     UnitList UL;
     int[] attachButton;
     int[] attachButtonB;
     int[] attachButtonUnr;
     int[] detachButton;
     int[] detachButtonUnr;

    pub TransportWindowClass( tGame: GameClass)
      : base( tGame, 1000, 760, 8)
    {
      self.attachButton = new int[30];
      self.attachButtonB = new int[30];
      self.attachButtonUnr = new int[30];
      self.detachButton = new int[30];
      self.detachButtonUnr = new int[30];
      self.SetUnits();
      self.View();
    }

    pub fn SetUnits()
    {
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
            {
              self.game.EditObj.TipButton = true;
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = self.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub fn View()
    {
      let mut x: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X;
      let mut y: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y;
      if (self.cancelid > 0)
      {
        self.RemoveSubPart(self.cancelid);
        self.cancelid = 0;
      }
      let mut index1: i32 = 0;
      do
      {
        self.attachButtonUnr[index1] = -1;
        if (self.attachButton[index1] > 0)
        {
          self.RemoveSubPart(self.attachButton[index1]);
          self.attachButton[index1] = 0;
        }
        if (self.attachButtonB[index1] > 0)
        {
          self.RemoveSubPart(self.attachButtonB[index1]);
          self.attachButtonB[index1] = 0;
        }
        self.detachButtonUnr[index1] = -1;
        if (self.detachButton[index1] > 0)
        {
          self.RemoveSubPart(self.detachButton[index1]);
          self.detachButton[index1] = 0;
        }
        index1 += 1;
      }
      while (index1 <= 29);
      self.ClearMouse();
      self.NewBackGroundAndClearAll(1000, 760, -1);
      Graphics toG = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawMessFrame( self.OwnBitmap,  toG, 0, 0, 1000, 760);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      let mut x1: i32 = 40;
      let mut w: i32 = 920;
      let mut h1: i32 = 40;
      let mut h2: i32 = 24;
      let mut y1_1: i32 = 20;
      let mut num1: i32 = 1;
      do
      {
        DrawMod.DrawBlockGradient2( toG, x1, y1_1, w, h2, self.game.MarcCol1, self.game.MarcCol2);
        tstring1: String;
        if (num1 == 1)
          tstring1 = "Transporter Unit";
        if (num1 == 2)
          tstring1 = "Attached Units";
        if (num1 == 3)
          tstring1 = "Other Units";
        DrawMod.DrawTextColouredMarc( toG, tstring1, self.game.MarcFont3, x1 + 20, y1_1 + 2, Color.White);
        let mut y1_2: i32 = y1_1 + h2;
        SimpleList simpleList = SimpleList::new();
        if (num1 == 1)
          simpleList.Add(self.game.EditObj.UnitSelected, 1);
        index2: i32;
        if (num1 == 2)
        {
          let mut unitCounter: i32 = self.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
          for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
          {
            index2 = self.game.Data.MapObj[0].HexObj[x, y].UnitList[index3];
            if (self.game.Data.UnitObj[index2].attachedTo == self.game.EditObj.UnitSelected)
              simpleList.Add(index2, 1);
          }
        }
        if (num1 == 3)
        {
          let mut unitCounter: i32 = self.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
          for (let mut index4: i32 = 0; index4 <= unitCounter; index4 += 1)
          {
            index2 = self.game.Data.MapObj[0].HexObj[x, y].UnitList[index4];
            let mut num2: i32 = 1;
            if (index2 == self.game.EditObj.UnitSelected)
              num2 = 0;
            if (!self.game.HandyFunctionsObj.CanAttach(index2))
              num2 = 0;
            if (num2 == 1)
              simpleList.Add(index2, 1);
          }
        }
        let mut num3: i32 = self.game.HandyFunctionsObj.GiveTransporterMaxCarry(self.game.EditObj.UnitSelected);
        let mut num4: i32 = self.game.HandyFunctionsObj.GiveTransporterCurrentCarry(self.game.EditObj.UnitSelected);
        let mut num5: i32 = self.game.HandyFunctionsObj.GiveTransporterMaxManpowerCarry(self.game.EditObj.UnitSelected);
        let mut num6: i32 = self.game.HandyFunctionsObj.GiveTransporterCurrentManpowerCarry(self.game.EditObj.UnitSelected);
        let mut counter: i32 = simpleList.Counter;
        for (let mut index5: i32 = 0; index5 <= counter; index5 += 1)
        {
          DrawMod.DrawBlock( toG, x1, y1_2, w, h1, 0, 0, 0, 64);
          self.Unr = simpleList.Id[index5];
          self.game.CustomBitmapObj.DrawUnit(self.Unr, toG: toG, tx: (x1 + 18), ty: (y1_2 + 2));
          name: String = self.game.Data.UnitObj[self.Unr].Name;
          DrawMod.DrawTextColouredMarc( toG, name, self.game.MarcFont4, x1 + 60, y1_2 + 5, Color.White);
          index2 = self.game.Data.UnitObj[self.Unr].HQ;
          tstring2: String = index2 <= -1 ? "no HQ" : "HQ: " + self.game.Data.UnitObj[index2].Name;
          DrawMod.DrawTextColouredMarc( toG, tstring2, self.game.MarcFont5, x1 + 62, y1_2 + 22, Color.LightGray);
          if (num1 == 1)
          {
            tstring3: String = "ATTACHED CARRY";
            DrawMod.DrawTextColouredMarcCenter( toG, tstring3, self.game.MarcFont5, x1 + 512 + 30, y1_2 + 5, Color.LightGray);
            tstring4: String = num4.ToString();
            DrawMod.DrawTextColouredMarcCenter( toG, tstring4, self.game.MarcFont4, x1 + 510 + 30, y1_2 + 17, Color.White);
            tstring5: String = "ATTACHED MANPOWER";
            DrawMod.DrawTextColouredMarcCenter( toG, tstring5, self.game.MarcFont5, x1 + 512 + 160, y1_2 + 5, Color.LightGray);
            tstring6: String = num6.ToString();
            DrawMod.DrawTextColouredMarcCenter( toG, tstring6, self.game.MarcFont4, x1 + 510 + 160, y1_2 + 17, Color.White);
            tstring7: String = "MAX WGT.CARRY";
            DrawMod.DrawTextColouredMarcCenter( toG, tstring7, self.game.MarcFont5, x1 + 312, y1_2 + 5, Color.LightGray);
            tstring8: String = num3.ToString();
            DrawMod.DrawTextColouredMarcCenter( toG, tstring8, self.game.MarcFont4, x1 + 310, y1_2 + 17, Color.White);
            tstring9: String = "MAX MANP.CARRY";
            DrawMod.DrawTextColouredMarcCenter( toG, tstring9, self.game.MarcFont5, x1 + 312 + 120, y1_2 + 5, Color.LightGray);
            tstring1 = num5.ToString();
            DrawMod.DrawTextColouredMarcCenter( toG, tstring1, self.game.MarcFont4, x1 + 310 + 120, y1_2 + 17, Color.White);
          }
          else
          {
            index2 = self.game.HandyFunctionsObj.GiveAttachablesWeight(self.Unr);
            tstring10: String = "WEIGHT";
            DrawMod.DrawTextColouredMarcCenter( toG, tstring10, self.game.MarcFont5, x1 + 512 + 30, y1_2 + 5, Color.LightGray);
            tstring11: String = index2.ToString();
            DrawMod.DrawTextColouredMarcCenter( toG, tstring11, self.game.MarcFont4, x1 + 510 + 30, y1_2 + 17, Color.White);
            index2 = self.game.HandyFunctionsObj.GiveAttachablesManpowerWeight(self.Unr);
            tstring12: String = "MANPOWER";
            DrawMod.DrawTextColouredMarcCenter( toG, tstring12, self.game.MarcFont5, x1 + 512 + 160, y1_2 + 5, Color.LightGray);
            tstring1 = index2.ToString();
            DrawMod.DrawTextColouredMarcCenter( toG, tstring1, self.game.MarcFont4, x1 + 510 + 160, y1_2 + 17, Color.White);
          }
          switch (num1)
          {
            case 2:
              int[] detachButton = self.detachButton;
              let mut index6: i32 = index5;
              let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Detach", 150, "Click to Detach Unit from the Transporter Unit.",  self.OwnBitmap, x1 + 750, y1_2 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
              let mut num7: i32 = self.AddSubPart( tsubpart1, x1 + 750, y1_2 + 5, 150, 35, 1);
              detachButton[index6] = num7;
              self.detachButtonUnr[index5] = self.Unr;
              break;
            case 3:
              index2 = self.game.HandyFunctionsObj.GiveAttachablesWeight(self.Unr);
              let mut num8: i32 = self.game.HandyFunctionsObj.GiveAttachablesManpowerWeight(self.Unr);
              if (index2 <= num3 - num4 & index2 > 0 & num8 <= num5 - num6)
              {
                int[] attachButton = self.attachButton;
                let mut index7: i32 = index5;
                let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Attach", 150, "Click to Attach Unit to the Transporter Unit.",  self.OwnBitmap, x1 + 750, y1_2 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
                let mut num9: i32 = self.AddSubPart( tsubpart2, x1 + 750, y1_2 + 5, 150, 35, 1);
                attachButton[index7] = num9;
                self.attachButtonUnr[index5] = self.Unr;
                break;
              }
              int[] attachButtonB = self.attachButtonB;
              let mut index8: i32 = index5;
              let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Attach", 150, "Cannot be attached.",  self.OwnBitmap, x1 + 750, y1_2 + 5, true, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
              let mut num10: i32 = self.AddSubPart( tsubpart3, x1 + 750, y1_2 + 5, 150, 35, 0);
              attachButtonB[index8] = num10;
              break;
          }
          y1_2 += h1;
        }
        y1_1 = y1_2 +  Math.Round( h2 / 2.0);
        num1 += 1;
      }
      while (num1 <= 3);
      let mut tsubpart: SubPartClass =  new TextButtonPartClass("OK", 250, "Click to return to main screen.",  self.OwnBitmap, 375, 700, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.cancelid = self.AddSubPart( tsubpart, 375, 700, 250, 40, 1);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height && self.MouseData[index] >= 0)
        {
          self.View();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
label_17:
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut index2: i32 = 0;
            while (!(self.SubPartID[index1] == self.attachButton[index2] & self.attachButtonUnr[index2] > -1))
            {
              if (self.SubPartID[index1] == self.detachButton[index2] & self.detachButtonUnr[index2] > -1)
              {
                self.game.ProcessingObj.DetachUnit(self.detachButtonUnr[index2], self.game.EditObj.UnitSelected);
                self.View();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              index2 += 1;
              if (index2 > 29)
              {
                if (self.SubPartID[index1] == self.cancelid)
                {
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                goto label_17;
              }
            }
            self.game.ProcessingObj.AttachUnit(self.attachButtonUnr[index2], self.game.EditObj.UnitSelected);
            self.View();
            windowReturnClass.SetFlag(true);
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
