// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AdviceWindow
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// using Microsoft.VisualBasic;
// using Microsoft.VisualBasic.CompilerServices;
// using System;
// using System.Drawing;
// using System.Runtime.CompilerServices;

// namespace WindowsApplication1
// {
  // pub class AdviceWindow : WindowClass
#[derive(Clone,Debug,Default)]
pub struct AdviceWindow
{
    pub hideId: i32,
    pub Info1Id: i32,
    pub info2id: i32,
    pub info3id: i32,
    pub info4id: i32,
    pub info5id: i32,
    pub info6id: i32,
    pub info7id: i32,
    pub info8id: i32,
    pub w: i32,
    pub h: i32,
    pub MouseOverWhichTab: i32,
    pub cacheList: String,
    pub profId: i32,
    pub currentShqNr: i32,
    pub special1id: i32,
    pub special2id: i32,
    pub special3id: i32,
}

impl AdviceWindow {

    // pub  AdviceWindow(
    //    tGame: GameClass,
    //    WindowClass tLowerWindow,
    //    Rectangle tLowerRect)
    //   : base( tGame, 820, 240, 8)
    pub fn new(tGame: GameClass, tLowerWindow: WindowClass, tLowerRect: Rectangle) -> Self
    {
      LowerWindow = tLowerWindow;
      LowerRect = tLowerRect;
      w = 820;
      h = 240;
      BlockBlit = false;
        Self {
            hideId: 0,
            Info1Id: 0,
            info2id: 0,
            info3id: 0,
            info4id: 0,
            info5id: 0,
            info6id: 0,
            info7id: 0,
            // LowerWindow: tLowerWindow,
            // LowerRect: tLowerRect,
            w: w,
            h: h,
            MouseOverWhichTab: 0,
            cacheList: (),
            profId: 0,
            currentShqNr: 0,
            special1id: 0,
            special2id: 0,
            // BlockBlit: false,
            info8id: 0,
            special3id: 0
        };
        dostuff();
        return Self;
    }

    // pub fn HandleMouseMove: WindowReturnClass(x: i32, y: i32)
      pub fn HandleMouseMove(mut self, x: i32, y: i32)
      {
      windowReturnClass: WindowReturnClass = base.HandleMouseMove(x, y);
      if self.game.EditObj.SetViewMode2 > 0 {
          windowReturnClass.Flag = false;
      }
      return windowReturnClass;
    }

    pub fn DoRefresh(mut self)  {self.dostuff();}

    // pub handleTimer: WindowReturnClass()
    pub fn handleTimer(mut self)
      {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.SetViewMode2 > 0 || self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0))].Length != -1)
        return windowReturnClass;
      self.game.EditObj.TempBlockAdvice = true;
      windowReturnClass.AddCommand(3, 11);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub fn dostuff(mut self)
    {
      let mut stringListById1 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0));
      let mut stringListById2 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 191, 0, 0));
      if (self.hideId > 0)
      {
        self.RemoveSubPart(self.hideId);
        self.hideId = 0;
      }
      if (self.Info1Id > 0)
      {
        self.RemoveSubPart(self.Info1Id);
        self.Info1Id = 0;
      }
      if (self.info2id > 0)
      {
        self.RemoveSubPart(self.info2id);
        self.info2id = 0;
      }
      if (self.info3id > 0)
      {
        self.RemoveSubPart(self.info3id);
        self.info3id = 0;
      }
      if (self.info4id > 0)
      {
        self.RemoveSubPart(self.info4id);
        self.info4id = 0;
      }
      if (self.info5id > 0)
      {
        self.RemoveSubPart(self.info5id);
        self.info5id = 0;
      }
      if (self.info6id > 0)
      {
        self.RemoveSubPart(self.info6id);
        self.info6id = 0;
      }
      if (self.info7id > 0)
      {
        self.RemoveSubPart(self.info7id);
        self.info7id = 0;
      }
      if (self.info8id > 0)
      {
        self.RemoveSubPart(self.info8id);
        self.info8id = 0;
      }
      if (self.special1id > 0)
      {
        self.RemoveSubPart(self.special1id);
        self.special1id = 0;
      }
      if (self.special2id > 0)
      {
        self.RemoveSubPart(self.special2id);
        self.special2id = 0;
      }
      if (self.special3id > 0)
      {
        self.RemoveSubPart(self.special3id);
        self.special3id = 0;
      }
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      if (self.game.EditObj.SetViewMode2 > 0)
      {
        self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      }
      else
      {
        let mut Expression: Graphics = Graphics.FromImage((Image) self.OwnBitmap);
         let mut local1: &Graphics =  &Expression;
        let mut bitmap = BitmapStore.GetBitmap(self.game.BACKGROUND3MARC);
         let mut local2 =  &bitmap;
        let mut rectangle1 = Rectangle::new(0, 0, 512, self.h - 24);
        let mut srcrect1 = &rectangle1;
        let mut rectangle2 = Rectangle::new(8, 8, 512, self.h - 24);
        let mut destrect1 = &rectangle2;
        DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
         let mut local3 =  Expression;
        let mut b = BitmapStore.GetBitmap(self.game.BACKGROUND3MARC);
         let mut local4 =  &b;
        rectangle2 = Rectangle::new(0, 0, 264, self.h - 24);
        let mut srcrect2: &Rectangle = &rectangle2;
        rectangle1 = Rectangle::new(520, 8, 264, self.h - 24);
        let mut destrect2: &Rectangle = &rectangle1;
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
        b = (Bitmap) null;
        DrawMod.DrawMessFrame( b,  Expression, 0, 0, self.w - 32, self.h - 16);
         let mut local5: &Graphics = &Expression;
        b = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_EXITLEFT);
         let mut local6: &Bitmap = &b;
        let mut x: i32 =  self.w - 32;
        let mut y: i32 =  self.h - 160;
        DrawMod.DrawSimple( local5,  local6, x, y);
        let mut tsubpart1: SubPartClass =  new SEButtonPartClass(self.game.SE1_ARROW2, "Hide the advice bar.", 23);
        self.hideId = self.AddSubPart( tsubpart1, self.w - 30, self.h - 160 + 18, 23, 35, 1);
        self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
        if (self.game.Data.StringListObj[stringListById1].Length > -1)
        {
          let mut id: i32 =  self.game.EventRelatedObj.CheckStringlistID("SE_IO", 158, 0, 0);
          if (self.game.EditObj.se1_adviceWindowPage < 0)
            self.game.EditObj.se1_adviceWindowPage = 0;
          if (self.game.EditObj.se1_adviceWindowPage > self.game.Data.StringListObj[stringListById1].Length)
            self.game.EditObj.se1_adviceWindowPage = self.game.Data.StringListObj[stringListById1].Length;
          let mut num1: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[self.game.EditObj.se1_adviceWindowPage, 0]));
          let mut charId: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[self.game.EditObj.se1_adviceWindowPage, 1]));
          let mut idValue1: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[self.game.EditObj.se1_adviceWindowPage, 2]));
          let mut idValue2: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[self.game.EditObj.se1_adviceWindowPage, 3]));
          str1: String = self.game.Data.StringListObj[stringListById1].Data[self.game.EditObj.se1_adviceWindowPage, 4];
          str2: String = self.game.Data.StringListObj[stringListById1].Data[self.game.EditObj.se1_adviceWindowPage, 5];
           let mut local7: &Graphics = &Expression;
          b = BitmapStore.GetBitmap(self.game.SE1_PAPER2);
           let mut local8: &Bitmap = &b;
          rectangle2 = Rectangle::new(16, 16, self.w - 58, self.h - 42);
          let mut srcrect3: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(14, 14, self.w - 58, self.h - 42);
          let mut destrect3: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local7,  local8, srcrect3, destrect3);
          tstring1: String = str2 + " " + (self.game.EditObj.se1_adviceWindowPage + 1).ToString() + "/" + (self.game.Data.StringListObj[stringListById1].Length + 1).ToString();
          DrawMod.DrawTextColouredConsoleCenter( Expression, tstring1, DrawMod.TGame.se1TypeWriterBig, (int) Math.Round( self.w / 2.0) - 18, 20, DrawMod.TGame.seColTW);
          if (charId > 0)
          {
             let mut local9: &Graphics = &Expression;
            b = self.game.CustomBitmapObj.DrawLeaderPortrait(charId, 78, 110);
             let mut local10: &Bitmap = &b;
            DrawMod.DrawSimple( local9,  local10, 20, 55);
            tstring2: String = self.game.EventRelatedObj.GetLeaderName(charId, true) + " says:";
            DrawMod.DrawTextColouredConsole( Expression, tstring2, DrawMod.TGame.se1TypeWriterSmall, 104, 52, DrawMod.TGame.seColTW);
          }
          self.game.EventRelatedObj.IO_AddClear();
          self.game.EventRelatedObj.IO_AddText("\"" + str1 + "\"", 0, 0, self.w - 182, IO_ColNr.Black, IO_FontNr.Regular, returnHeight: true);
          let mut tsubpart2: SubPartClass =  new UDSPartClass(self.game, self.w - 182, self.h - 48, self.game.EventRelatedObj.CheckKey(id, "FINALTEXT", 0, 0),  self.OwnBitmap, 104, 69, true);
          self.info4id = self.AddSubPart( tsubpart2, 104, 69, self.w - 182, self.h - 48, 0);
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Del", 50, "Never give me this specific type of advice again for remainder of the game.",  self.OwnBitmap, 16, self.h - 65, usefont: DrawMod.TGame.se1TypeWriterMedium, tudsButton: true);
          self.info5id = self.AddSubPart( tsubpart3, 16, self.h - 65, 50, 35, 1);
          let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Dismiss", 120, "Remove this advice for now, but remind me later if deemed neccessary.",  self.OwnBitmap, 66, self.h - 65, usefont: DrawMod.TGame.se1TypeWriterMedium, tudsButton: true);
          self.info6id = self.AddSubPart( tsubpart4, 66, self.h - 65, 120, 35, 1);
          if (idValue1 > 0)
          {
            str3: String = self.game.Data.StringListObj[stringListById2].GetData(0, idValue1, 2);
            let mut num2: i32 =  Strings.InStr(str3.ToLower(), "How to".ToLower());
            if (num2 > 0)
              str3 = Strings.Mid(str3, num2 + 7);
            let mut num3: i32 =  Strings.InStr(str3, " ");
            if (num3 > 0)
            {
              let mut num4: i32 =  Strings.InStr(num3 + 1, str3, " ");
              if (num4 > 0)
              {
                let mut num5: i32 =  Strings.InStr(num4 + 1, str3, " ");
                if (num5 > 0)
                {
                  let mut num6: i32 =  Strings.InStr(num5 + 1, str3, " ");
                  if (num6 > 0)
                    str3 = Strings.Left(str3, num6 - 1);
                }
              }
            }
            let mut tsubpart5: SubPartClass =  new TextButtonPartClass(Strings.Left(str3, 1).ToUpper() + Strings.Mid(str3, 2), 290, "Give more information on: " + self.game.Data.StringListObj[stringListById2].GetData(0, idValue1, 2),  self.OwnBitmap, 186, self.h - 65, usefont: DrawMod.TGame.se1TypeWriterMedium, tudsButton: true);
            self.info7id = self.AddSubPart( tsubpart5, 186, self.h - 65, 290, 35, 1);
          }
          if (idValue2 > 0)
          {
            str4: String = self.game.Data.StringListObj[stringListById2].GetData(0, idValue2, 2);
            let mut num7: i32 =  Strings.InStr(str4.ToLower(), "How to".ToLower());
            if (num7 > 0)
              str4 = Strings.Mid(str4, num7 + 7);
            let mut num8: i32 =  Strings.InStr(str4, " ");
            if (num8 > 0)
            {
              let mut num9: i32 =  Strings.InStr(num8 + 1, str4, " ");
              if (num9 > 0)
              {
                let mut num10: i32 =  Strings.InStr(num9 + 1, str4, " ");
                if (num10 > 0)
                {
                  let mut num11: i32 =  Strings.InStr(num10 + 1, str4, " ");
                  if (num11 > 0)
                    str4 = Strings.Left(str4, num11 - 1);
                }
              }
            }
            let mut tsubpart6: SubPartClass =  new TextButtonPartClass(Strings.Left(str4, 1).ToUpper() + Strings.Mid(str4, 2), 290, "Give more information on: " + self.game.Data.StringListObj[stringListById2].GetData(0, idValue2, 2),  self.OwnBitmap, 481, self.h - 65, usefont: DrawMod.TGame.se1TypeWriterMedium, tudsButton: true);
            self.info8id = self.AddSubPart( tsubpart6, 481, self.h - 65, 290, 35, 1);
          }
          if (self.game.Data.StringListObj[stringListById1].Length > 0)
          {
            let mut num12: i32 =  0;
            if (self.game.EditObj.se1_adviceWindowPage < 1)
              num12 = 1;
            let mut tsubpart7: SubPartClass =  new TextButtonPartClass("<", 60, "Go to previous Advice...",  self.OwnBitmap, 30, 20, num12 == 1, theight: 30, usefont: DrawMod.TGame.se1TypeWriterMedium, tudsButton: true);
            self.info2id = self.AddSubPart( tsubpart7, 30, 20, 60, 30, 1);
            let mut num13: i32 =  1;
            if (self.game.EditObj.se1_adviceWindowPage < self.game.Data.StringListObj[stringListById1].Length)
              num13 = 0;
            let mut tsubpart8: SubPartClass =  new TextButtonPartClass(">", 60, "Go to next Advice...",  self.OwnBitmap, self.w - 110, 20, num13 == 1, theight: 30, usefont: DrawMod.TGame.se1TypeWriterMedium, tudsButton: true);
            self.info3id = self.AddSubPart( tsubpart8, self.w - 110, 20, 60, 30, 1);
          }
        }
        if (Information.IsNothing( Expression))
          return;
        Expression.Dispose();
        Expression = (Graphics) null;
      }
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  self.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.game.EditObj.TipButton = false;
            self.SubPartList[index].HandleToolTip(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (self.game.EditObj.TipButton)
              return;
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
      let mut mouseCounter: i32 =  self.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          if (Strings.InStr(self.game.EditObj.TipText, "MX-ENTR") <= 0)
            break;
          self.game.EditObj.TipTitle += "<FIXEDSYS>";
          break;
        }
      }
    }

    pub fn PopUpRefresh() => self.DoRefresh();

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.SetViewMode2 > 0 || self.game.EditObj.TutOrder > -1)
        return windowReturnClass;
      let mut subPartCounter: i32 =  self.SubPartCounter;
      for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
      {
        if (self.SubPartCounter > -1 && x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          let mut num1: i32 =  self.SubPartID[index];
          if (num1 == self.Info1Id)
          {
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.info2id)
          {
            --self.game.EditObj.se1_adviceWindowPage;
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.hideId)
          {
            self.game.EditObj.BlockAdvice = true;
            self.game.EditObj.TempBlockAdvice = true;
            windowReturnClass.AddCommand(3, 11);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.info3id)
          {
            this += 1.game.EditObj.se1_adviceWindowPage;
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.info7id)
          {
            let mut stringListById: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0));
            self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 503, 0, 0));
            let mut num2: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[self.game.EditObj.se1_adviceWindowPage, 0]));
            let mut idValue: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[self.game.EditObj.se1_adviceWindowPage, 2]));
            self.game.EditObj.udsManagementTabOverrideId = (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Present", 79, 0, 0))].GetData(11, idValue, 0)));
            self.game.EditObj.SetViewMode2 = 12;
            self.ClearMouse();
            self.NewBackGroundAndClearAll(self.w, self.h, -1);
            self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
            if (self.game.ScreenHeight < 920 & self.game.ScreenWidth < 1465)
            {
              self.game.EditObj.GuiDown = true;
              self.game.EditObj.RightDown = true;
              windowReturnClass.AddCommand(3, 11);
            }
            else if (self.game.ScreenHeight < 920)
            {
              self.game.EditObj.GuiDown = true;
              windowReturnClass.AddCommand(3, 11);
            }
            else if (self.game.ScreenWidth < 1465)
            {
              self.game.EditObj.RightDown = true;
              windowReturnClass.AddCommand(3, 11);
            }
            else
              windowReturnClass.AddCommand(3, 11);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.info8id)
          {
            let mut stringListById: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0));
            self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 503, 0, 0));
            let mut num3: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[self.game.EditObj.se1_adviceWindowPage, 0]));
            let mut idValue: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[self.game.EditObj.se1_adviceWindowPage, 3]));
            self.game.EditObj.udsManagementTabOverrideId = (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Present", 79, 0, 0))].GetData(11, idValue, 0)));
            self.game.EditObj.SetViewMode2 = 12;
            self.ClearMouse();
            self.NewBackGroundAndClearAll(self.w, self.h, -1);
            self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
            if (self.game.ScreenHeight < 920 & self.game.ScreenWidth < 1465)
            {
              self.game.EditObj.GuiDown = true;
              self.game.EditObj.RightDown = true;
              windowReturnClass.AddCommand(3, 11);
            }
            else if (self.game.ScreenHeight < 920)
            {
              self.game.EditObj.GuiDown = true;
              windowReturnClass.AddCommand(3, 11);
            }
            else if (self.game.ScreenWidth < 1465)
            {
              self.game.EditObj.RightDown = true;
              windowReturnClass.AddCommand(3, 11);
            }
            else
              windowReturnClass.AddCommand(3, 11);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.info5id)
          {
            let mut stringListById1: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0));
            let mut stringListById2: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 503, 0, 0));
            let mut idValue: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[self.game.EditObj.se1_adviceWindowPage, 0]));
            if (idValue > 0)
            {
              let mut num4: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, idValue, 1)));
              let mut num5: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, idValue, 2)));
              let mut setValue: i32 =  999999;
              self.game.Data.StringListObj[stringListById2].SetData(0, idValue, 1, setValue, true);
              self.game.Data.StringListObj[stringListById1].RemoveRow(self.game.EditObj.se1_adviceWindowPage);
            }
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (num1 == self.info6id)
          {
            let mut stringListById3: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 502, 0, 0));
            let mut stringListById4: i32 =  self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 503, 0, 0));
            let mut idValue: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].Data[self.game.EditObj.se1_adviceWindowPage, 0]));
            if (idValue > 0)
            {
              let mut num6: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, idValue, 1)));
              let mut num7: i32 =  (int) Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, idValue, 2)));
              if (num7 < 1)
                num7 = 1;
              let mut setValue1: i32 =  num7 + (1 + num7);
              let mut setValue2: i32 =  self.game.Data.Round + (1 + setValue1);
              self.game.Data.StringListObj[stringListById4].SetData(0, idValue, 1, setValue2, true);
              self.game.Data.StringListObj[stringListById4].SetData(0, idValue, 2, setValue1, true);
              self.game.Data.StringListObj[stringListById3].RemoveRow(self.game.EditObj.se1_adviceWindowPage);
            }
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      let mut mouseCounter: i32 =  self.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height && self.MouseData[index] > 0 & self.MouseData[index] <= 3)
        {
          self.dostuff();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (x > 0 & x < self.w && y > 0 & y < self.h)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false) => WindowReturnClass::new();
  }
// }
