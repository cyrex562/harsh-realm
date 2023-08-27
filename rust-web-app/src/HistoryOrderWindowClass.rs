// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HistoryOrderWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class HistoryOrderWindowClass : WindowClass
  {
     bool TimerUsed;
     w: i32;
     h: i32;
     CurrentView: i32;
     info2id: i32;
     exitid: i32;
     skipid: i32;
     slider1id: i32;
     specialid: i32;
     autoplayid: i32;
     HistoryWindowClass2 HisW;

    pub HistoryOrderWindowClass(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
       HistoryWindowClass2 tHisW)
      : base( tGame, tGame.ScreenWidth, 90)
    {
      this.NewGfx = true;
      if (!this.game.EditObj.AIMoving)
      {
        this.game.EditObj.RealRound = this.game.Data.Round;
        this.game.EditObj.RealTurn = this.game.Data.Turn;
      }
      this.w = tGame.ScreenWidth;
      this.h = 90;
      this.LowerWindow = tLowerWindow;
      this.LowerRect = tLowerRect;
      this.HisW = tHisW;
      this.dostuff();
    }

    pub fn DoRefresh() => this.dostuff();

    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index] && Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
          {
            this.game.EditObj.TipButton = true;
            this.game.EditObj.TipTitle = "";
            this.game.EditObj.TipText = this.SubPartList[index].Descript;
            return;
          }
        }
      }
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          break;
        }
      }
    }

    pub fn dostuff()
    {
      this.CurrentView = this.game.EditObj.SetViewMode;
      this.ClearMouse();
      if (this.info2id > 0)
      {
        this.RemoveSubPart(this.info2id);
        this.info2id = 0;
      }
      if (this.exitid > 0)
      {
        this.RemoveSubPart(this.exitid);
        this.exitid = 0;
      }
      if (this.slider1id > 0)
      {
        this.RemoveSubPart(this.slider1id);
        this.slider1id = 0;
      }
      if (this.skipid > 0)
        this.RemoveSubPart(this.skipid);
      if (this.specialid > 0)
        this.RemoveSubPart(this.specialid);
      if (this.autoplayid > 0)
        this.RemoveSubPart(this.autoplayid);
      this.NewBackGroundAndClearAll(this.w, this.h, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      let mut num1: i32 =   Math.Round( this.game.ScreenWidth / 116.0);
      bitmap: Bitmap;
      for (let mut index: i32 =  0; index <= num1; index += 1)
      {
         let mut local1: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(this.game.MARCBUTBARFRAME);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  index * 116;
        DrawMod.DrawSimple( local1,  local2, x, 87);
      }
      let mut num2: i32 =   Math.Round( (this.game.ScreenWidth - 1152) / 2.0);
      let mut num3: i32 =  -3;
       let mut local3: &Graphics = &Expression;
      bitmap = BitmapStore.GetBitmap(this.game.MARCBUTBARHISTORY);
       let mut local4: &Bitmap = &bitmap;
      let mut x1: i32 =  num2;
      let mut y: i32 =  num3;
      DrawMod.DrawSimple( local3,  local4, x1, y);
      let mut num4: i32 =  34;
      let mut num5: i32 =  num2 + 40;
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      if (this.game.AIRunning | this.game.se1ThreadRunning)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("AI Playing!", this.game.MarcFont1, 200, 37, false, toutline: true, tMarc: true);
        this.info2id = this.AddSubPart( tsubpart, num5 - 15, num4 - 5, 185, 37, 0);
      }
      else if (this.game.AIRunning & !this.game.se1ThreadRunning & this.game.EditObj.AIMoving)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("AI Finished!", this.game.MarcFont1, 200, 37, false, toutline: true, tMarc: true);
        this.info2id = this.AddSubPart( tsubpart, num5 - 15, num4 - 5, 185, 37, 0);
      }
      else
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("History Log", this.game.MarcFont1, 200, 37, false, toutline: true, tMarc: true);
        this.info2id = this.AddSubPart( tsubpart, num5 - 15, num4 - 5, 185, 37, 0);
      }
      if (this.HisW.StartStep > -1 & this.HisW.EndStep > -1)
      {
        let mut tsubpart: SubPartClass =  new NumberSliderSubPartClass2(this.game, "Step", " of " + Conversion.Str( Math.Max(Math.Max(0, this.HisW.Curstep), Math.Max(0, this.HisW.EndStep))), 550, 0, Math.Max(0, this.HisW.EndStep), Math.Max(0, this.HisW.Curstep), tbackbitmap: ( this.OwnBitmap), bbx: (num5 + 170), bby: (num4 - 14), tMarc: true);
        this.slider1id = this.AddSubPart( tsubpart, num5 + 170, num4 - 14, 550, 40, 0);
      }
      if (this.HisW.Curstep >= this.HisW.EndStep)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("BACK", 100, "Rewind to first recorded move.",  this.OwnBitmap, num5 + 690 + 130, num4, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.specialid = this.AddSubPart( tsubpart, num5 + 690 + 130, num4, 100, 35, 1);
      }
      else
      {
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("NEXT BATTLE", 120, "Click to fast-forward to the next battle.",  this.OwnBitmap, num5 + 690 + 130, num4, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.specialid = this.AddSubPart( tsubpart1, num5 + 690 + 130, num4, 120, 35, 1);
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("NEXT REGIME", 120, "Click to fast-forward to the moves of the next regime.",  this.OwnBitmap, num5 + 960, num4, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.skipid = this.AddSubPart( tsubpart2, num5 + 960, num4, 120, 35, 1);
      }
      if (!this.HisW.AutoPlay)
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("PLAY", 60, "Click to start Autoplay [shortkey P]",  this.OwnBitmap, num5 + 610 + 130, num4, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.autoplayid = this.AddSubPart( tsubpart, num5 + 610 + 130, num4, 60, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("STOP", 60, "Click to pause Autoplay [shortkey P]",  this.OwnBitmap, num5 + 610 + 130, num4, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.autoplayid = this.AddSubPart( tsubpart, num5 + 610 + 130, num4, 60, 35, 1);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (y > 18 &&  this.w / 2.0 - 500.0 <  x &  x <  this.w / 2.0 + 500.0)
        windowReturnClass.NoMouseClickBelow = true;
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.skipid)
            {
              this.game.EditObj.TempCoordList = CoordList::new();
              if (this.HisW.Curstep == 0 & this.HisW.Curstep < this.HisW.EndStep)
              {
                this.HisW.Forward(1);
                this += 1.HisW.Curstep;
              }
              if (this.HisW.lastregime >= -1)
              {
                let mut lastregime1: i32 =  this.HisW.lastregime;
                let mut num2: i32 =  1;
                while (num2 == 1)
                {
                  num2 = 0;
                  if (this.HisW.Curstep < this.HisW.EndStep)
                  {
                    this.HisW.Forward(1);
                    this += 1.HisW.Curstep;
                    num2 = 1;
                    let mut lastregime2: i32 =  this.HisW.lastregime;
                    if (lastregime2 != lastregime1 & lastregime2 != -1)
                      num2 = 0;
                  }
                }
              }
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 80);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 81);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.slider1id)
            {
              let mut steps: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index], b);
              if (steps > this.HisW.Curstep)
              {
                this.game.EditObj.TempCoordList = CoordList::new();
                this.HisW.Forward(steps - this.HisW.Curstep);
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 80);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 81);
                windowReturnClass.AddCommand(4, 9);
                this.HisW.Curstep = steps;
              }
              else if (steps < this.HisW.Curstep)
              {
                this.HisW.StartSit();
                this.HisW.Forward(steps);
                this.game.EditObj.TempCoordList = CoordList::new();
                windowReturnClass.AddCommand(4, 12);
                windowReturnClass.AddCommand(4, 80);
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 81);
                windowReturnClass.AddCommand(4, 9);
                this.HisW.Curstep = steps;
              }
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.specialid)
            {
              if (this.HisW.Curstep >= this.HisW.EndStep)
              {
                this.HisW.StartSit();
                this.game.EditObj.TempCoordList = CoordList::new();
                this.HisW.Curstep = 0;
              }
              else
              {
                do
                {
                  this.HisW.Forward(1);
                  this.game.EditObj.TempCoordList = CoordList::new();
                  this += 1.HisW.Curstep;
                }
                while (this.game.EditObj.HisAttackType == -1 & this.HisW.Curstep < this.HisW.EndStep);
              }
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 80);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 81);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.exitid)
            {
              if (this.HisW.HumanPlayer > -1)
              {
                if (this.game.Data.UseAI == 0)
                  this.game.AIObj.CloseAI();
                if (this.game.Data.UseAI == 1)
                  this.game.NewAIObj.CloseAI();
                windowReturnClass.SetFlag(true);
                this.game.EditObj.AIMoving = false;
                if (this.game.Data.RegimeObj[this.game.EditObj.RealTurn].AI)
                  this.game.EventRelatedObj.DoCheckEvents(5);
                windowReturnClass.AddCommand(3, 13);
                this.game.EditObj.OrderType = 0;
                this.game.EditObj.TempCoordList = CoordList::new();
                return windowReturnClass;
              }
              this.game.EditObj.TempCoordList = CoordList::new();
              if (this.game.Data.RegimeObj[this.game.EditObj.RealTurn].AI)
                this.game.EventRelatedObj.DoCheckEvents(5);
              windowReturnClass.AddCommand(3, 11);
              this.game.EditObj.OrderType = 0;
              windowReturnClass.SetFlag(true);
              windowReturnClass.SetFlag(true);
            }
            else if (num1 == this.autoplayid)
            {
              this.HisW.AutoPlay = !this.HisW.AutoPlay;
              if (this.HisW.AutoPlay & this.HisW.Curstep >= this.HisW.EndStep)
              {
                this.HisW.StartSit();
                this.HisW.Curstep = 0;
              }
              windowReturnClass.AddCommand(4, 80);
              this.dostuff();
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
