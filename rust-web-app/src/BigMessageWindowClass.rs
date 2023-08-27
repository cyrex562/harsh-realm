// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.BigMessageWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class BigMessageWindowClass : WindowClass
  {
     int okid;
     int ok2id;
     int tbacknr;
     int oktextid;
     int noteid;
     int note2id;
     int cloudid;
     int orderid;
     int orderidb;
     int Pic1Id;
     int TAid;
     int detailnr;
     int detailnr2;
     int OptionsList5id;
     ListClass OptionsList5Obj;
     int OptionsList6id;
     ListClass OptionsList6Obj;
     int FromMessage;
     int coreMessage;
     DynamicData dyn;

    pub BigMessageWindowClass(ref tGame: GameClass)
      : base(ref tGame, 960, 750, 8)
    {
      this.FromMessage = tGame.EditObj.FromMessage;
      this.coreMessage = this.FromMessage;
      this.detailnr2 = -1;
      this.detailnr = -1;
      this.ViewMessage();
    }

    pub fn HandleToolTip(int x, int y)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            this.game.EditObj.TipButton = false;
            this.SubPartList[index].HandleToolTip(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (this.game.EditObj.TipButton)
              return;
            this.SubPartList[index].DescriptInfo(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
            {
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = "";
              this.game.EditObj.TipText = this.SubPartList[index].Descript;
              return;
            }
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

    pub fn ViewMessage()
    {
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.ok2id > 0)
      {
        this.RemoveSubPart(this.ok2id);
        this.ok2id = 0;
      }
      if (this.orderid > 0)
      {
        this.RemoveSubPart(this.orderid);
        this.orderid = 0;
      }
      if (this.orderidb > 0)
      {
        this.RemoveSubPart(this.orderid);
        this.orderidb = 0;
      }
      if (this.TAid > 0)
      {
        this.RemoveSubPart(this.TAid);
        this.TAid = 0;
      }
      this.ClearMouse();
      this.NewBackGroundAndClearAll(960, 750, 8);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawMessFrame(ref this.OwnBitmap, ref graphics, 0, 0, 960, 750);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      if (this.FromMessage == -1)
        return;
      DrawMod.DrawBlockGradient(ref graphics, 660, 40, 150, 420, Color.FromArgb(96, 0, 0, 0), Color.FromArgb(0, 0, 0, 0));
      DrawMod.drawLineDot(ref graphics, 660, 40, 660, 460, Color.White);
      DrawMod.drawLineDot(ref graphics, 660, 460, 860, 460, Color.White);
      if (Strings.Len(this.game.Data.RegimeObj[this.game.Data.Turn].MessWav[this.FromMessage]) > 0)
      {
        SoundMod.STopEventWave();
        SoundMod.PlayEventWave(this.game.AppPath + "sound/" + this.game.Data.RegimeObj[this.game.Data.Turn].MessWav[this.FromMessage], ref this.game.EditObj);
      }
      this.dyn = new DynamicData(this.game.Data.RegimeObj[this.game.Data.Turn].MessString[this.FromMessage]);
      this.OptionsList5Obj = ListClass::new();
      let mut tlistselect1: i32 =  -1;
      let mut num1: i32 =  -1;
      if (this.game.Data.RegimeObj[this.game.Data.Turn].MesChosen[this.coreMessage] > 0)
      {
        let mut elementCounter: i32 =  this.dyn.elementCounter;
        for (let mut tdata: i32 =  0; tdata <= elementCounter; tdata += 1)
        {
          if (this.dyn.element[tdata].type == DynamicType.OptionField & tdata == this.game.Data.RegimeObj[this.game.Data.Turn].MesChosen[this.coreMessage])
          {
            num1 += 1;
            if (this.detailnr == -1)
              this.detailnr = tdata;
            if (this.detailnr == tdata)
              tlistselect1 = num1;
            tname: String;
            if (this.dyn.element[tdata].optionpp > 0)
              tname = "DECIDED: " + this.dyn.element[tdata].optiontitle + "#" + this.dyn.element[tdata].optiontext + "#" + this.dyn.element[tdata].optionpp.ToString() + "PP".to_owned();
            else
              tname = "DECIDED: " + this.dyn.element[tdata].optiontitle + "#" + this.dyn.element[tdata].optiontext + "#" + this.dyn.element[tdata].optionpp.ToString() + "PP".to_owned();
            this.OptionsList5Obj.add(tname, tdata);
          }
        }
      }
      else
      {
        let mut elementCounter: i32 =  this.dyn.elementCounter;
        for (let mut tdata: i32 =  0; tdata <= elementCounter; tdata += 1)
        {
          if (this.dyn.element[tdata].type == DynamicType.OptionField)
          {
            num1 += 1;
            if (this.detailnr == -1)
              this.detailnr = tdata;
            if (this.detailnr == tdata)
              tlistselect1 = num1;
            tname: String;
            if (this.dyn.element[tdata].optionpp > 0)
              tname = this.dyn.element[tdata].optiontitle + "#" + this.dyn.element[tdata].optiontext + "#" + this.dyn.element[tdata].optionpp.ToString() + "PP".to_owned();
            else
              tname = this.dyn.element[tdata].optiontitle + "#" + this.dyn.element[tdata].optiontext + "#" + this.dyn.element[tdata].optionpp.ToString() + "PP".to_owned();
            this.OptionsList5Obj.add(tname, tdata);
          }
        }
      }
      if (this.OptionsList5Obj.ListCount > -1)
      {
        if (this.OptionsList5id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList5id)].Refresh(this.OptionsList5Obj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsList5id)] = true;
        }
        else
        {
          let mut tsubpart: SubPartClass =  new BigListSubPartClass(this.OptionsList5Obj, 4, 565, tlistselect1, this.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 50, bby: 550, tMarcStyle: true);
          this.OptionsList5id = this.AddSubPart(ref tsubpart, 50, 550, 565, 160, 0);
        }
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("CLOSE", 150, "Click to close this message popup.", ref this.OwnBitmap, 680, 400, theight: 30, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.okid = this.AddSubPart(ref tsubpart1, 680, 400, 150, 30, 1);
      let mut tsubpart2: SubPartClass =  new MarcButtonPartClass(this.game.BACKBUTTON, tDescript: "Exit this message popup [ESC]", tBackbitmap: (ref this.OwnBitmap), bbx: 894, bby: 24);
      this.ok2id = this.AddSubPart(ref tsubpart2, 894, 24, 32, 32, 1);
      int num2;
      let mut num3: i32 =  (int) Math.Round(Conversion.Int( (270 - num2) / 16.0));
      let mut num4: i32 =  0;
      if (this.OptionsList5Obj.ListCount == -1)
        num4 = 120;
      if (this.game.Data.RegimeObj[this.game.Data.Turn].MesStyle[this.FromMessage] == 3)
      {
        let mut tsubpart3: SubPartClass =  new DynamicArea(this.game, 565, 510 + num4, this.game.Data.RegimeObj[this.game.Data.Turn].MessString[this.FromMessage], ref this.OwnBitmap, 50, 20);
        this.TAid = this.AddSubPart(ref tsubpart3, 50, 20, 565, 510 + num4, 0);
      }
      else
      {
        let mut tsubpart4: SubPartClass =  new TextAreaClass2(this.game, 565, (int) Math.Round(Conversion.Int( (510 + num4 - 17) / 17.0)), this.game.MarcFont8, "[tab]Message," + this.game.Data.RegimeObj[this.game.Data.Turn].MessString[this.FromMessage] + "[/tab]", 17, ref this.OwnBitmap, 50, 20, tUseEncy: true);
        this.TAid = this.AddSubPart(ref tsubpart4, 50, 20, 565, (int) Math.Round(Conversion.Int( (510 + num4 - 17) / 17.0) * 17.0), 0);
      }
      if (this.OptionsList5Obj.ListCount > -1)
      {
        if (this.game.Data.RegimeObj[this.game.Data.Turn].MesChosen[this.coreMessage] > 0)
        {
          let mut tsubpart5: SubPartClass =  new TextButtonPartClass("DECIDE (" + this.dyn.element[this.detailnr].optionpp.ToString() + "PP)", 250, "You have already made a decision.", ref this.OwnBitmap, 640, 605, true, theight: 60, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.orderidb = this.AddSubPart(ref tsubpart5, 640, 605, 250, 60, 0);
        }
        else if (this.dyn.element[this.detailnr].optionpp <= this.game.Data.RegimeObj[this.game.Data.Turn].ResPts)
        {
          let mut tsubpart6: SubPartClass =  new TextButtonPartClass("DECIDE (" + this.dyn.element[this.detailnr].optionpp.ToString() + "PP)", 250, "Click to make the selected decision.", ref this.OwnBitmap, 640, 605, theight: 60, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.orderid = this.AddSubPart(ref tsubpart6, 640, 605, 250, 60, 1);
        }
        else
        {
          let mut tsubpart7: SubPartClass =  new TextButtonPartClass("DECIDE (" + this.dyn.element[this.detailnr].optionpp.ToString() + "PP)", 250, "You do not have the PP needed to take this decision.", ref this.OwnBitmap, 640, 605, true, theight: 60, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
          this.orderidb = this.AddSubPart(ref tsubpart7, 640, 605, 250, 60, 0);
        }
      }
      else
      {
        let mut tsubpart8: SubPartClass =  new TextButtonPartClass("DECIDE", 250, "No decision available with report you are viewing right now.", ref this.OwnBitmap, 640, 605, true, theight: 60, usefont: this.game.MarcFont3, useshadow: true, tMarcStyle: true);
        this.orderidb = this.AddSubPart(ref tsubpart8, 640, 605, 250, 60, 0);
      }
      this.OptionsList6Obj = ListClass::new();
      let mut tlistselect2: i32 =  -1;
      let mut num5: i32 =  -1;
      if (this.detailnr2 == -1)
        this.detailnr2 = this.coreMessage;
      let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
      for (let mut tdata: i32 =  0; tdata <= messCounter; tdata += 1)
      {
        if (this.game.Data.RegimeObj[this.game.Data.Turn].MesGroup[tdata] > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].MesGroup[tdata] == this.game.Data.RegimeObj[this.game.Data.Turn].MesGroup[this.coreMessage])
        {
          num5 += 1;
          if (this.detailnr2 == -1)
            this.detailnr2 = 0;
          if (this.detailnr2 == tdata)
            tlistselect2 = num5;
          tname: String = this.game.Data.RegimeObj[this.game.Data.Turn].MesName[tdata].Length <= 1 ? this.game.Data.RegimeObj[this.game.Data.Turn].MessString[tdata] : this.game.Data.RegimeObj[this.game.Data.Turn].MesName[tdata];
          bool flag = false;
          DynamicData dynamicData = new DynamicData(this.game.Data.RegimeObj[this.game.Data.Turn].MessString[tdata]);
          let mut elementCounter: i32 =  dynamicData.elementCounter;
          for (let mut index: i32 =  0; index <= elementCounter; index += 1)
          {
            if (dynamicData.element[index].type == DynamicType.OptionField)
              flag = true;
          }
          if (flag)
          {
            if (this.game.Data.RegimeObj[this.game.Data.Turn].MesChosen[tdata] == 0)
              this.OptionsList6Obj.add(tname, tdata, tbmp: BitmapStore.GetBitmap(this.game.SMALLCHAR1));
            else
              this.OptionsList6Obj.add(tname, tdata, tbmp: BitmapStore.GetBitmap(this.game.SMALLCHAR2));
          }
          else
            this.OptionsList6Obj.add(tname, tdata);
        }
      }
      if (this.OptionsList6Obj.ListCount > 0)
      {
        DrawMod.DrawTextColouredMarc(ref graphics, "REPORT BUNDLE", this.game.MarcFont4, 680, 175, Color.White);
        if (this.OptionsList6id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList6id)].Refresh(this.OptionsList6Obj, tlistselect2);
          this.SubPartFlag[this.SubpartNr(this.OptionsList6id)] = true;
        }
        else
        {
          ListClass optionsList6Obj = this.OptionsList6Obj;
          let mut tlistselect3: i32 =  tlistselect2;
          let mut game: GameClass = this.game;
          ref local1: Bitmap = ref this.OwnBitmap;
          font: Font =  null;
          ref local2: Font = ref font;
          let mut tsubpart9: SubPartClass =  new ListSubPartClass(optionsList6Obj, 9, 180, tlistselect3, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: (ref local1), bbx: 680, bby: 200, tMarcStyle: true, overruleFont: (ref local2));
          this.OptionsList6id = this.AddSubPart(ref tsubpart9, 680, 200, 180, 160, 0);
        }
      }
      DrawMod.DrawTextColouredMarc(ref graphics, "POLITICAL POINTS", this.game.MarcFont4, 680, 102, Color.White);
      let mut num6: i32 =  680;
      let mut y: i32 =  125;
      DrawMod.DrawBlockGradient2(ref graphics, num6 + 5, y - 1, 79, 20, this.game.MarcCol1, this.game.MarcCol2);
      DrawMod.DrawFrame(ref this.OwnBitmap, ref this.BackBitmap, ref graphics, num6 + 5, y, 80, 19, -1, -1);
      str: String = Strings.Trim(Conversion.Str( Math.Round(new Decimal(this.game.Data.RegimeObj[this.game.Data.Turn].ResPts))));
      DrawMod.DrawTextColouredMarc(ref graphics, str + "PP", this.game.MarcFont5, num6 + 10, y + 4, Color.White);
      Rectangle trect = Rectangle::new(num6 + 10, y, 80, 20);
      this.AddMouse(ref trect, "Available political points", Conversions.ToString(-1));
    }

    pub HandleKeyPress: WindowReturnClass(int nr, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      int index;
      switch (nr)
      {
        case 27:
          SoundMod.STopEventWave();
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 38:
          this.SubPartList[this.SubpartNr(this.OptionsList6id)].ShiftUp();
          this.detailnr2 = this.SubPartList[this.SubpartNr(this.OptionsList6id)].GetSelect();
          this.RemoveSubPart(this.TAid);
          this.TAid = 0;
          this.RemoveSubPart(this.OptionsList5id);
          this.OptionsList5id = 0;
          this.FromMessage = this.detailnr2;
          this.SubPartFlag[this.SubpartNr(this.OptionsList6id)] = true;
          this.SubPartFlag[index] = true;
          this.ViewMessage();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        case 40:
          this.SubPartList[this.SubpartNr(this.OptionsList6id)].ShiftDown();
          this.detailnr2 = this.SubPartList[this.SubpartNr(this.OptionsList6id)].GetSelect();
          this.RemoveSubPart(this.TAid);
          this.TAid = 0;
          this.RemoveSubPart(this.OptionsList5id);
          this.OptionsList5id = 0;
          this.FromMessage = this.detailnr2;
          this.SubPartFlag[this.SubpartNr(this.OptionsList6id)] = true;
          this.SubPartFlag[index] = true;
          this.ViewMessage();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        default:
          return windowReturnClass;
      }
    }

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.TAid)
            {
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.orderid)
            {
              SoundMod.STopEventWave();
              let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
              RegimeClass[] regimeObj = this.game.Data.RegimeObj;
              RegimeClass[] regimeClassArray = regimeObj;
              let mut turn: i32 =  this.game.Data.Turn;
              let mut index2: i32 =  turn;
              regimeClassArray[index2].ResPts = regimeObj[turn].ResPts - this.dyn.element[this.detailnr].optionpp;
              this.game.Data.RegimeObj[this.game.Data.Turn].MesChosen[this.detailnr2] = this.detailnr;
              this.game.EditObj.AreaX = -1;
              this.game.EditObj.AreaY = -1;
              this.game.EventRelatedObj.DoCheckSpecificEvent(this.dyn.element[this.detailnr].optionevent, this.dyn.element[this.detailnr].optiontempvar[0], this.dyn.element[this.detailnr].optiontempvar[1], this.dyn.element[this.detailnr].optiontempvar[4], this.dyn.element[this.detailnr].optiontempvar[5], this.dyn.element[this.detailnr].optiontempvar[9], this.dyn.element[this.detailnr].optiontempvar[7], this.dyn.element[this.detailnr].optiontempvar[8], this.dyn.element[this.detailnr].optiontempvar[10]);
              if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
              {
                this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                this.game.EditObj.PopupValue = 0;
                windowReturnClass.AddCommand(5, 14);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList5id)
            {
              let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.ViewMessage();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList6id)
            {
              let mut num3: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                this.detailnr2 = num3;
                this.RemoveSubPart(this.TAid);
                this.TAid = 0;
                this.RemoveSubPart(this.OptionsList5id);
                this.OptionsList5id = 0;
                this.FromMessage = this.detailnr2;
                this.ViewMessage();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.okid || num1 == this.ok2id)
            {
              SoundMod.STopEventWave();
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
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
