// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.EventWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class EventWindowClass : WindowClass
  {
     Info1textId: i32;
     info1id: i32;
     info2textid: i32;
     info2id: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     AddEventId: i32;
     RemoveEventId: i32;
     EventUpId: i32;
     EventDownId: i32;
     GroupListId: i32;
     ListClass GroupListObj;
     object groupdetail;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     info3textid: i32;
     info3id: i32;
     AddCommandId: i32;
     RemoveCommandId: i32;
     CommandUpId: i32;
     CommandDownId: i32;
     CommandCopy: i32;
     CommandPaste: i32;
     EventCopy: i32;
     EventPaste: i32;
     setCat: i32;
     setPrio: i32;
     setPrioB: i32;
     DescriptId: i32;
     CurTyp: i32;
     Typ1: i32;
     Typ1b: i32;
     Typ2: i32;
     Typ2b: i32;
     Typ3: i32;
     Typ3b: i32;
     Typ4: i32;
     Typ4b: i32;
     Typ5: i32;
     Typ5b: i32;
     Typ6: i32;
     Typ6b: i32;
     Typ7: i32;
     Typ7b: i32;
     TypMode: i32;
     TypModeB: i32;
     typstr: i32;
     typstrb: i32;
     Export: i32;
     ExportB: i32;
     Import1: i32;
     Import2: i32;
     replaceId: i32;
     Item1: i32;
     Item1b: i32;
     cat: i32;
     Item2: i32;
     Item2b: i32;
     Item3: i32;
     Item3b: i32;
     Item4: i32;
     Item4b: i32;
     Item5: i32;
     Item5b: i32;
     ITEMNAME: Vec<String>;
     OptionsList3Id: i32;
     ListClass OptionsList3Obj;
     OptionsList4Id: i32;
     ListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ListClass OptionsList5Obj;
     OptionsList6Id: i32;
     ListClass OptionsList6Obj;
     Tab1: i32;
     Tab2: i32;
     Tab3: i32;
     Tab4: i32;
     Tab5: i32;
     Tab6: i32;
     Tab7: i32;
     Tab8: i32;
     Tab9: i32;
     Tab10: i32;
     tab110: i32;
     tab110b: i32;
     tab111: i32;
     tab111b: i32;
     NTab1: i32;
     NTab2: i32;
     NTab1b: i32;
     NTab2b: i32;
     ntab1x: i32;
     ntab2x: i32;
     setLib: i32;
     Tab1b: i32;
     Tab2b: i32;
     Tab3b: i32;
     Tab4b: i32;
     Tab5b: i32;
     Tab6b: i32;
     Tab7b: i32;
     Tab8b: i32;
     Tab9b: i32;
     Tab10b: i32;
     tab11: i32;
     tab12: i32;
     tab13: i32;
     tab131: i32;
     tab132: i32;
     tab133: i32;
     tab14: i32;
     tab15: i32;
     tab16: i32;
     tab17: i32;
     tab18: i32;
     tab11b: i32;
     tab12b: i32;
     tab13b: i32;
     tab131b: i32;
     tab132b: i32;
     tab133b: i32;
     tab14b: i32;
     tab15b: i32;
     tab16b: i32;
     tab17b: i32;
     tab18b: i32;
     tab19b: i32;
     tab19: i32;
     tabx1: i32;
     tabx2: i32;
     tabx3: i32;
     quick1: i32;
     quick1b: i32;
     quick2: i32;
     quick2b: i32;
     quick3: i32;
     quick3b: i32;
     quick4: i32;
     quick4b: i32;
     quick5: i32;
     quick5b: i32;
     quick6: i32;
     quick6b: i32;
     quick7: i32;
     quick7b: i32;
     quick8: i32;
     quick8b: i32;
     int[] tbutton;
     int[] tline;
     int[] toption;
     InsertId: i32;
     allowExecuteId: i32;
     int[] trefer;
     tok: i32;
     tok2: i32;
     tta: i32;
     temptext: Vec<String>;
     int[] tempoption;
     int[] temprefer;
     bool DoEscape;
     ExecNow: i32;
     detail1: i32;
     detail2: i32;
     detail3: i32;
     detail4: i32;
     detailchoice: i32;
     detailLib: i32;
     COMMANDTYPE: Vec<String>;
     String1: String;
     String2: String;
     String3: String;
     bool DoingSlots;
     SetSlot: i32;
     StepCurrent: i32;
     VarExtra: i32;
     const let mut VARINFO: i32 =  1;
     const let mut CHECKTYPE: i32 =  2;
     const let mut CONDITION: i32 =  3;
     const let mut EXECTYPE: i32 =  4;
     const let mut SETTYPE: i32 =  5;
     const let mut STRINGINFO: i32 =  6;
     const let mut CHECK: i32 =  1;
     const let mut ENDCHECK: i32 =  2;
     const let mut EXEC: i32 =  3;
     const let mut SETVAR: i32 =  4;
     const let mut LOOPER: i32 =  5;
     const let mut ENDLOOPER: i32 =  6;
     const let mut COMMENT: i32 =  7;
     ss: String;

    pub EventWindowClass(ref tGame: GameClass)
      : base(ref tGame, tGame.ScreenWidth, tGame.ScreenHeight)
    {
      this.ITEMNAME = new string[6];
      this.tbutton = new int[20];
      this.tline = new int[20];
      this.toption = new int[20];
      this.trefer = new int[20];
      this.temptext = new string[20];
      this.tempoption = new int[20];
      this.temprefer = new int[20];
      this.COMMANDTYPE = new string[11];
      this.detail1 = -1;
      this.detail2 = -1;
      this.detail3 = -1;
      this.detail4 = -1;
      this.groupdetail =  -1;
      this.cat = -1;
      this.COMMANDTYPE[0] = "EMPTY".to_owned();
      this.COMMANDTYPE[1] = nameof (CHECK);
      this.COMMANDTYPE[2] = "END CHECK";
      this.COMMANDTYPE[3] = "EXECUTE".to_owned();
      this.COMMANDTYPE[4] = nameof (SETVAR);
      this.COMMANDTYPE[5] = nameof (LOOPER);
      this.COMMANDTYPE[6] = "END LOOPER";
      this.COMMANDTYPE[7] = nameof (COMMENT);
      this.DoingSlots = false;
      this.dostuff();
    }

    pub fn DoRefresh() => this.dostuff();

    pub fn bigclear()
    {
      if (this.OptionsList3Id > 0)
      {
        this.RemoveSubPart(this.OptionsList3Id);
        this.OptionsList3Id = 0;
      }
      if (this.GroupListId > 0)
      {
        this.RemoveSubPart(this.GroupListId);
        this.GroupListId = 0;
      }
      if (this.DescriptId <= 0)
        return;
      this.RemoveSubPart(this.DescriptId);
      this.DescriptId = 0;
    }

    pub fn dostuff()
    {
      if (this.info1id > 0)
        this.RemoveSubPart(this.info1id);
      if (this.Info1textId > 0)
        this.RemoveSubPart(this.Info1textId);
      if (this.info2textid > 0)
        this.RemoveSubPart(this.info2textid);
      if (this.info3id > 0)
        this.RemoveSubPart(this.info3id);
      if (this.setLib > 0)
        this.RemoveSubPart(this.setLib);
      if (this.info3textid > 0)
        this.RemoveSubPart(this.info3textid);
      if (this.OptionsListId > 0)
        this.RemoveSubPart(this.OptionsListId);
      if (this.OptionsList2Id > 0)
        this.RemoveSubPart(this.OptionsList2Id);
      if (this.OptionsList4Id > 0)
        this.RemoveSubPart(this.OptionsList4Id);
      if (this.OptionsList5Id > 0)
        this.RemoveSubPart(this.OptionsList5Id);
      if (this.OptionsList6Id > 0)
        this.RemoveSubPart(this.OptionsList6Id);
      if (this.AddEventId > 0)
        this.RemoveSubPart(this.AddEventId);
      if (this.RemoveEventId > 0)
        this.RemoveSubPart(this.RemoveEventId);
      if (this.AddCommandId > 0)
        this.RemoveSubPart(this.AddCommandId);
      if (this.RemoveCommandId > 0)
        this.RemoveSubPart(this.RemoveCommandId);
      if (this.CommandCopy > 0)
        this.RemoveSubPart(this.CommandCopy);
      if (this.CommandPaste > 0)
        this.RemoveSubPart(this.CommandPaste);
      if (this.EventCopy > 0)
        this.RemoveSubPart(this.EventCopy);
      if (this.EventPaste > 0)
        this.RemoveSubPart(this.EventPaste);
      if (this.setPrio > 0)
        this.RemoveSubPart(this.setPrio);
      if (this.setPrioB > 0)
        this.RemoveSubPart(this.setPrioB);
      if (this.Typ1 > 0)
        this.RemoveSubPart(this.Typ1);
      if (this.Typ1b > 0)
        this.RemoveSubPart(this.Typ1b);
      if (this.Typ2 > 0)
        this.RemoveSubPart(this.Typ2);
      if (this.Typ2b > 0)
        this.RemoveSubPart(this.Typ2b);
      if (this.Typ3 > 0)
        this.RemoveSubPart(this.Typ3);
      if (this.Typ3b > 0)
        this.RemoveSubPart(this.Typ3b);
      if (this.Typ4 > 0)
        this.RemoveSubPart(this.Typ4);
      if (this.Typ4b > 0)
        this.RemoveSubPart(this.Typ4b);
      if (this.Typ5 > 0)
        this.RemoveSubPart(this.Typ5);
      if (this.Typ5b > 0)
        this.RemoveSubPart(this.Typ5b);
      if (this.Typ6 > 0)
        this.RemoveSubPart(this.Typ6);
      if (this.Typ6b > 0)
        this.RemoveSubPart(this.Typ6b);
      if (this.Typ7 > 0)
        this.RemoveSubPart(this.Typ7);
      if (this.Typ7b > 0)
        this.RemoveSubPart(this.Typ7b);
      if (this.CurTyp > 0)
        this.RemoveSubPart(this.CurTyp);
      if (this.Item1 > 0)
        this.RemoveSubPart(this.Item1);
      if (this.Item1b > 0)
        this.RemoveSubPart(this.Item1b);
      if (this.Item2 > 0)
        this.RemoveSubPart(this.Item2);
      if (this.Item2b > 0)
        this.RemoveSubPart(this.Item2b);
      if (this.Item3 > 0)
        this.RemoveSubPart(this.Item3);
      if (this.Item3b > 0)
        this.RemoveSubPart(this.Item3b);
      if (this.quick1 > 0)
        this.RemoveSubPart(this.quick1);
      if (this.quick1b > 0)
        this.RemoveSubPart(this.quick1b);
      if (this.quick2 > 0)
        this.RemoveSubPart(this.quick2);
      if (this.quick2b > 0)
        this.RemoveSubPart(this.quick2b);
      if (this.quick3 > 0)
        this.RemoveSubPart(this.quick3);
      if (this.quick3b > 0)
        this.RemoveSubPart(this.quick3b);
      if (this.quick4 > 0)
        this.RemoveSubPart(this.quick4);
      if (this.quick4b > 0)
        this.RemoveSubPart(this.quick4b);
      if (this.quick5 > 0)
        this.RemoveSubPart(this.quick5);
      if (this.quick5b > 0)
        this.RemoveSubPart(this.quick5b);
      if (this.quick6 > 0)
        this.RemoveSubPart(this.quick6);
      if (this.quick6b > 0)
        this.RemoveSubPart(this.quick6b);
      if (this.quick7 > 0)
        this.RemoveSubPart(this.quick7);
      if (this.quick7b > 0)
        this.RemoveSubPart(this.quick7b);
      if (this.quick8 > 0)
        this.RemoveSubPart(this.quick8);
      if (this.quick8b > 0)
        this.RemoveSubPart(this.quick8b);
      if (this.Tab1 > 0)
        this.RemoveSubPart(this.Tab1);
      if (this.Tab1b > 0)
        this.RemoveSubPart(this.Tab1b);
      if (this.Tab2 > 0)
        this.RemoveSubPart(this.Tab2);
      if (this.Tab2b > 0)
        this.RemoveSubPart(this.Tab2b);
      if (this.NTab1 > 0)
        this.RemoveSubPart(this.NTab1);
      if (this.NTab1b > 0)
        this.RemoveSubPart(this.NTab1b);
      if (this.NTab2 > 0)
        this.RemoveSubPart(this.NTab2);
      if (this.NTab2b > 0)
        this.RemoveSubPart(this.NTab2b);
      if (this.Tab3 > 0)
        this.RemoveSubPart(this.Tab3);
      if (this.Tab3b > 0)
        this.RemoveSubPart(this.Tab3b);
      if (this.Tab4 > 0)
        this.RemoveSubPart(this.Tab4);
      if (this.Tab4b > 0)
        this.RemoveSubPart(this.Tab4b);
      if (this.Tab5 > 0)
        this.RemoveSubPart(this.Tab5);
      if (this.Tab5b > 0)
        this.RemoveSubPart(this.Tab5b);
      if (this.Tab6 > 0)
        this.RemoveSubPart(this.Tab6);
      if (this.Tab6b > 0)
        this.RemoveSubPart(this.Tab6b);
      if (this.Tab7 > 0)
        this.RemoveSubPart(this.Tab7);
      if (this.Tab7b > 0)
        this.RemoveSubPart(this.Tab7b);
      if (this.Tab8 > 0)
        this.RemoveSubPart(this.Tab8);
      if (this.Tab8b > 0)
        this.RemoveSubPart(this.Tab8b);
      if (this.Tab9 > 0)
        this.RemoveSubPart(this.Tab9);
      if (this.Tab9b > 0)
        this.RemoveSubPart(this.Tab9b);
      if (this.Tab10 > 0)
        this.RemoveSubPart(this.Tab10);
      if (this.Tab10b > 0)
        this.RemoveSubPart(this.Tab10b);
      if (this.tab11 > 0)
        this.RemoveSubPart(this.tab11);
      if (this.tab11b > 0)
        this.RemoveSubPart(this.tab11b);
      if (this.tab110 > 0)
        this.RemoveSubPart(this.tab110);
      if (this.tab110b > 0)
        this.RemoveSubPart(this.tab110b);
      if (this.tab111 > 0)
        this.RemoveSubPart(this.tab111);
      if (this.tab111b > 0)
        this.RemoveSubPart(this.tab111b);
      if (this.tab12 > 0)
        this.RemoveSubPart(this.tab12);
      if (this.tab12b > 0)
        this.RemoveSubPart(this.tab12b);
      if (this.tab13 > 0)
        this.RemoveSubPart(this.tab13);
      if (this.tab13b > 0)
        this.RemoveSubPart(this.tab13b);
      if (this.tab131 > 0)
        this.RemoveSubPart(this.tab131);
      if (this.tab131b > 0)
        this.RemoveSubPart(this.tab131b);
      if (this.tab132 > 0)
        this.RemoveSubPart(this.tab132);
      if (this.tab132b > 0)
        this.RemoveSubPart(this.tab132b);
      if (this.tab133 > 0)
        this.RemoveSubPart(this.tab133);
      if (this.tab133b > 0)
        this.RemoveSubPart(this.tab133b);
      if (this.tab14 > 0)
        this.RemoveSubPart(this.tab14);
      if (this.tab14b > 0)
        this.RemoveSubPart(this.tab14b);
      if (this.tab15 > 0)
        this.RemoveSubPart(this.tab15);
      if (this.tab15b > 0)
        this.RemoveSubPart(this.tab15b);
      if (this.tab16 > 0)
        this.RemoveSubPart(this.tab16);
      if (this.tab16b > 0)
        this.RemoveSubPart(this.tab16b);
      if (this.tab17 > 0)
        this.RemoveSubPart(this.tab17);
      if (this.tab17b > 0)
        this.RemoveSubPart(this.tab17b);
      if (this.tab18 > 0)
        this.RemoveSubPart(this.tab18);
      if (this.tab18b > 0)
        this.RemoveSubPart(this.tab18b);
      if (this.tab19 > 0)
        this.RemoveSubPart(this.tab19);
      if (this.tab19b > 0)
        this.RemoveSubPart(this.tab19b);
      if (this.tabx1 > 0)
        this.RemoveSubPart(this.tabx1);
      if (this.tabx2 > 0)
        this.RemoveSubPart(this.tabx2);
      if (this.tabx3 > 0)
        this.RemoveSubPart(this.tabx3);
      if (this.ntab1x > 0)
        this.RemoveSubPart(this.ntab1x);
      if (this.ntab2x > 0)
        this.RemoveSubPart(this.ntab2x);
      if (this.allowExecuteId > 0)
        this.RemoveSubPart(this.allowExecuteId);
      if (this.TypMode > 0)
        this.RemoveSubPart(this.TypMode);
      if (this.TypModeB > 0)
        this.RemoveSubPart(this.TypModeB);
      if (this.Export > 0)
        this.RemoveSubPart(this.Export);
      if (this.ExportB > 0)
        this.RemoveSubPart(this.ExportB);
      if (this.typstr > 0)
        this.RemoveSubPart(this.typstr);
      if (this.typstrb > 0)
        this.RemoveSubPart(this.typstrb);
      if (this.CommandUpId > 0)
        this.RemoveSubPart(this.CommandUpId);
      if (this.CommandDownId > 0)
        this.RemoveSubPart(this.CommandDownId);
      if (this.EventUpId > 0)
        this.RemoveSubPart(this.EventUpId);
      if (this.EventDownId > 0)
        this.RemoveSubPart(this.EventDownId);
      if (this.setCat > 0)
        this.RemoveSubPart(this.setCat);
      if (this.Import1 > 0)
        this.RemoveSubPart(this.Import1);
      if (this.Import2 > 0)
        this.RemoveSubPart(this.Import2);
      if (this.ExecNow > 0)
        this.RemoveSubPart(this.ExecNow);
      if (this.replaceId > 0)
        this.RemoveSubPart(this.replaceId);
      if (this.InsertId > 0)
        this.RemoveSubPart(this.InsertId);
      let mut index1: i32 =  0;
      do
      {
        if (this.tline[index1] > 0)
          this.RemoveSubPart(this.tline[index1]);
        if (this.toption[index1] > 0)
          this.RemoveSubPart(this.toption[index1]);
        if (this.trefer[index1] > 0)
          this.RemoveSubPart(this.trefer[index1]);
        if (this.tbutton[index1] > 0)
          this.RemoveSubPart(this.tbutton[index1]);
        index1 += 1;
      }
      while (index1 <= 19);
      if (this.tok > 0)
        this.RemoveSubPart(this.tok);
      if (this.tok2 > 0)
        this.RemoveSubPart(this.tok2);
      if (this.tta > 0)
        this.RemoveSubPart(this.tta);
      if (this.DoingSlots)
        this.TabsheetSlots();
      if (this.DoingSlots)
        return;
      this.ss = "click to exit the event screen";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
      this.info1id = this.AddSubPart(ref tsubpart1, 10, 2, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("Exit Event Screen", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
      this.Info1textId = this.AddSubPart(ref tsubpart2, 50, 2, 200, 20, 0);
      this.OptionsList6Obj = ListClass::new();
      this.OptionsList6Obj.add("All Events", -1);
      let mut num1: i32 =  0;
      let mut libraryCounter: i32 =  this.game.Data.LibraryCounter;
      str1: String;
      for (let mut index2: i32 =  0; index2 <= libraryCounter; index2 += 1)
      {
        str1 = this.game.Data.LibraryObj[index2].name;
        this.OptionsList6Obj.add(str1, index2 + 1);
        if (index2 == this.detailLib)
          num1 = index2 + 1;
      }
      ListClass optionsList6Obj = this.OptionsList6Obj;
      let mut tlistselect1: i32 =  num1;
      let mut game1: GameClass = this.game;
      ref local1: Bitmap = ref this.OwnBitmap;
      font1: Font =  null;
      ref local2: Font = ref font1;
      let mut tsubpart3: SubPartClass =  new ListSubPartClass(optionsList6Obj, 17, 190, tlistselect1, game1, true, "Libraries", tbackbitmap: (ref local1), bbx: 10, bby: 230, overruleFont: (ref local2));
      this.OptionsList6Id = this.AddSubPart(ref tsubpart3, 10, 230, 190, 320, 0);
      this.OptionsList5Obj = ListClass::new();
      this.OptionsList5Obj.add("All Events", -1);
      let mut num2: i32 =  0;
      let mut num3: i32 =  0;
      let mut tdata: i32 =  0;
      do
      {
        if (Strings.Len(this.game.Data.TempString[800 + tdata]) > 1)
        {
          num3 += 1;
          str1 = this.game.Data.TempString[800 + tdata];
          if (tdata == this.cat)
            num2 = num3;
          this.OptionsList5Obj.add(str1, tdata);
        }
        tdata += 1;
      }
      while (tdata <= 99);
      ListClass optionsList5Obj = this.OptionsList5Obj;
      let mut tlistselect2: i32 =  num2;
      let mut game2: GameClass = this.game;
      ref local3: Bitmap = ref this.OwnBitmap;
      font2: Font =  null;
      ref local4: Font = ref font2;
      let mut tsubpart4: SubPartClass =  new ListSubPartClass(optionsList5Obj, 7, 190, tlistselect2, game2, true, "Event Categories", tbackbitmap: (ref local3), bbx: 10, bby: 20, overruleFont: (ref local4));
      this.OptionsList5Id = this.AddSubPart(ref tsubpart4, 10, 20, 190, 160, 0);
      this.OptionsListObj = ListClass::new();
      let mut num4: i32 =  -1;
      let mut num5: i32 =  -1;
      if (this.game.Data.EventCounter > -1)
      {
        let mut eventCounter: i32 =  this.game.Data.EventCounter;
        for (let mut index3: i32 =  0; index3 <= eventCounter; index3 += 1)
        {
          if ((this.cat == -1 | this.game.Data.EventObj[index3].Category == this.cat) & (this.detailLib == -1 | this.game.Data.EventObj[index3].LibId.libSlot == this.detailLib))
          {
            num4 += 1;
            if (index3 == this.detail1)
              num5 = num4;
            str2: String = "slot" + Strings.Trim(Conversion.Str( index3)) + ") " + this.game.Data.EventObj[index3].Name + "<id" + this.game.Data.EventObj[index3].Id.ToString() + ">";
            if (Strings.Len(str2) > 60)
              str2 = Strings.Left(str2, 60);
            str3: String = str2 + Strings.Space(65 - Strings.Len(str2));
            if (this.game.Data.EventObj[index3].CheckMode == 0)
              str3 += "No Check";
            else if (this.game.Data.EventObj[index3].CheckMode == 1)
              str3 += "Round Check";
            else if (this.game.Data.EventObj[index3].CheckMode == 2)
              str3 += "Early Turn (no unter)";
            else if (this.game.Data.EventObj[index3].CheckMode == 3)
              str3 += "Late Turn Check";
            else if (this.game.Data.EventObj[index3].CheckMode == 4)
              str3 += "Late Open Turn Check";
            else if (this.game.Data.EventObj[index3].CheckMode == 5)
              str3 += "Close Turn Check";
            else if (this.game.Data.EventObj[index3].CheckMode == 6)
              str3 += "After AI Init Check";
            else if (this.game.Data.EventObj[index3].CheckMode == 7)
              str3 += "Game Startup Always";
            else if (this.game.Data.EventObj[index3].CheckMode == 8)
              str3 += "Early Open Turn Check (depr)";
            else if (this.game.Data.EventObj[index3].CheckMode == 9)
              str3 += "Load Game Early (by button)";
            else if (this.game.Data.EventObj[index3].CheckMode == 10)
              str3 += "Load Game Late";
            else if (this.game.Data.EventObj[index3].CheckMode == 11)
              str3 += "Mid Turn Check (no unter)";
            else if (this.game.Data.EventObj[index3].CheckMode == 12)
              str3 += "Early Cinematics Check";
            if (this.game.Data.EventObj[index3].Priority != 0)
              str3 = str3 + ", prio=" + this.game.Data.EventObj[index3].Priority.ToString();
            if (Strings.Len(str3) > 102)
              str3 = Strings.Left(str3, 102);
            str4: String = str3 + Strings.Space(105 - Strings.Len(str3));
            if (this.game.Data.EventObj[index3].Category > -1)
              str4 += this.game.Data.TempString[800 + this.game.Data.EventObj[index3].Category];
            if (Strings.Len(str4) > 132)
              str4 = Strings.Left(str4, 132);
            str1 = str4 + Strings.Space(135 - Strings.Len(str4));
            if (this.game.Data.EventObj[index3].LibId.libSlot > -1)
              str1 = str1 + this.game.Data.LibraryObj[this.game.Data.EventObj[index3].LibId.libSlot].name + " <libId" + this.game.Data.EventObj[index3].LibId.id.ToString() + ">";
            if (Strings.Len(str1) > 160)
              str1 = Strings.Left(str1, 160);
            this.OptionsListObj.add(str1, index3);
          }
        }
      }
      if (this.game.Data.EventCounter < this.detail1)
        this.detail1 = -1;
      let mut w1: i32 =  this.game.ScreenWidth - 400;
      ListClass optionsListObj = this.OptionsListObj;
      let mut twidth1: i32 =  w1;
      let mut tlistselect3: i32 =  num5;
      let mut game3: GameClass = this.game;
      ref local5: Bitmap = ref this.OwnBitmap;
      font3: Font =  null;
      ref local6: Font = ref font3;
      let mut tsubpart5: SubPartClass =  new ListSubPartClass(optionsListObj, 7, twidth1, tlistselect3, game3, true, "Events", tbackbitmap: (ref local5), bbx: 210, bby: 20, overruleFont: (ref local6));
      this.OptionsListId = this.AddSubPart(ref tsubpart5, 210, 20, w1, 160, 0);
      this.ss = "Click to add a new event to the eventlist";
      let mut tsubpart6: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
      this.AddEventId = this.AddSubPart(ref tsubpart6, 250 + w1, 20, 32, 16, 1);
      if (this.detail1 > -1)
      {
        this.ss = "Click to move this event up in the list";
        let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONUP, tDescript: this.ss);
        this.EventUpId = this.AddSubPart(ref tsubpart7, 250 + w1, 40, 32, 16, 1);
        this.ss = "Click to move this event down in the list";
        let mut tsubpart8: SubPartClass =  ButtonPartClass::new(this.game.BUTTONDOWN, tDescript: this.ss);
        this.EventDownId = this.AddSubPart(ref tsubpart8, 250 + w1, 60, 32, 16, 1);
        this.ss = "Click to make a copy of this event in memory";
        let mut tsubpart9: SubPartClass =  ButtonPartClass::new(this.game.BUTTONCOPY, tDescript: this.ss);
        this.EventCopy = this.AddSubPart(ref tsubpart9, 250 + w1, 80, 32, 16, 1);
        this.ss = "Click to add the event below the selected event";
        let mut tsubpart10: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPASTE, tDescript: this.ss);
        this.EventPaste = this.AddSubPart(ref tsubpart10, 250 + w1, 100, 32, 16, 1);
        this.ss = "Click to set category";
        let mut tsubpart11: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.setCat = this.AddSubPart(ref tsubpart11, 250 + w1, 120, 32, 16, 1);
        this.ss = "Click to set library";
        let mut tsubpart12: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.setLib = this.AddSubPart(ref tsubpart12, 320 + w1, 120, 32, 16, 1);
        this.ss = "Replace Options ; Do not forget to make a backup first. ";
        let mut tsubpart13: SubPartClass =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
        this.replaceId = this.AddSubPart(ref tsubpart13, 320 + w1, 140, 32, 16, 1);
        this.ss = "Click to export all events to textfiles in log directory";
        let mut tsubpart14: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
        this.Export = this.AddSubPart(ref tsubpart14, 320 + w1, 20, 32, 16, 1);
      }
      this.ss = "Click to import a specific (you must know slot number) event from another file";
      let mut tsubpart15: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
      this.Import1 = this.AddSubPart(ref tsubpart15, 320 + w1, 40, 32, 16, 1);
      this.ss = "Click to import ALL or a specific category of events from another file";
      let mut tsubpart16: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
      this.Import2 = this.AddSubPart(ref tsubpart16, 320 + w1, 60, 32, 16, 1);
      this.ss = "Click to execute this event right now...";
      let mut tsubpart17: SubPartClass =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.ExecNow = this.AddSubPart(ref tsubpart17, 320 + w1, 90, 32, 16, 1);
      y: i32;
      SubPartClass tsubpart18;
      if (this.detail1 > -1)
      {
        this.ss = "Click to remove this event from the list";
        let mut tsubpart19: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.RemoveEventId = this.AddSubPart(ref tsubpart19, 250 + w1, 140, 32, 16, 1);
        let mut w2: i32 =  this.game.ScreenWidth - 260;
        y = this.game.ScreenHeight - 420;
        this.ss = "Click to give this event a new name";
        let mut tsubpart20: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.info3id = this.AddSubPart(ref tsubpart20, 10, 202, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("Event: " + this.game.Data.EventObj[this.detail1].Name, Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.info3textid = this.AddSubPart(ref tsubpart18, 50, 202, 200, 20, 0);
        this.OptionsList2Obj = ListClass::new();
        let mut num6: i32 =  0;
        if (this.game.Data.EventObj[this.detail1].CommandCounter > -1)
        {
          let mut commandCounter: i32 =  this.game.Data.EventObj[this.detail1].CommandCounter;
          for (let mut index4: i32 =  0; index4 <= commandCounter; index4 += 1)
          {
            str5: String = this.COMMANDTYPE[this.game.Data.EventObj[this.detail1].CommandList[index4].type];
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 2)
            {
              --num6;
              if (0 > num6)
                num6 = 0;
            }
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 6)
            {
              --num6;
              if (0 > num6)
                num6 = 0;
            }
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 1)
              str5 = str5 + ": " + this.ItemInfo(this.detail1, index4, 0) + " " + this.ItemInfo(this.detail1, index4, 1) + " " + this.ItemInfo(this.detail1, index4, 2);
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 3)
              str5 = str5 + ": " + this.ItemInfo(this.detail1, index4, 0);
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 7)
              str5 = "' " + this.game.Data.EventObj[this.detail1].CommandList[index4].DataString;
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 4)
              str5 = str5 + ": " + this.ItemInfo(this.detail1, index4, 0) + " " + this.ItemInfo(this.detail1, index4, 1) + " " + this.ItemInfo(this.detail1, index4, 2);
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 5)
              str5 = str5 + ": " + this.ItemInfo(this.detail1, index4, 0) + " " + this.ItemInfo(this.detail1, index4, 1) + " " + this.ItemInfo(this.detail1, index4, 2);
            str1 = Strings.Trim(Conversion.Str( index4)) + ") " + Strings.Space(4 - Strings.Len(Strings.Trim(Conversion.Str( index4)))) + Strings.Space(num6 * 2) + str5;
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 7)
              this.OptionsList2Obj.add(str1, index4, tcol: 1);
            else
              this.OptionsList2Obj.add(str1, index4);
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 1)
              num6 += 1;
            if (this.game.Data.EventObj[this.detail1].CommandList[index4].type == 5)
              num6 += 1;
          }
        }
        if (this.game.Data.EventObj[this.detail1].CommandCounter < this.detail2)
          this.detail2 = -1;
        ListClass optionsList2Obj = this.OptionsList2Obj;
        let mut tlistsize: i32 =   Math.Round(Conversion.Int( y / 16.0) - 1.0);
        let mut twidth2: i32 =  w2;
        let mut detail2: i32 =  this.detail2;
        let mut game4: GameClass = this.game;
        ref local7: Bitmap = ref this.OwnBitmap;
        font4: Font =  null;
        ref local8: Font = ref font4;
        tsubpart18 =  new ListSubPartClass(optionsList2Obj, tlistsize, twidth2, detail2, game4, true, "Code", tbackbitmap: (ref local7), bbx: 210, bby: 220, overruleFont: (ref local8));
        this.OptionsList2Id = this.AddSubPart(ref tsubpart18, 210, 220, w2,  Math.Round((Conversion.Int( y / 16.0) + 2.0) * 16.0), 0);
        this.ss = "Click to add a line of code to the selected event; will be placed below selected line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.AddCommandId = this.AddSubPart(ref tsubpart18, 300, 202, 32, 16, 1);
      }
      else
      {
        this.detail2 = -1;
        this.detail3 = -1;
        this.detail4 = -1;
      }
      if (this.detail2 > -1)
      {
        this.ss = "Click to remove this line of code";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.RemoveCommandId = this.AddSubPart(ref tsubpart18, 550, 202, 32, 16, 1);
        this.ss = "Click to move this line of code up one line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONUP, tDescript: this.ss);
        this.CommandUpId = this.AddSubPart(ref tsubpart18, 400, 202, 32, 16, 1);
        this.ss = "Click to move this line of code down one line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONDOWN, tDescript: this.ss);
        this.CommandDownId = this.AddSubPart(ref tsubpart18, 435, 202, 32, 16, 1);
        this.ss = "Click to copy this line into memory";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONCOPY, tDescript: this.ss);
        this.CommandCopy = this.AddSubPart(ref tsubpart18, 470, 202, 32, 16, 1);
        this.ss = "Click to paste the line in memory below the selected line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONPASTE, tDescript: this.ss);
        this.CommandPaste = this.AddSubPart(ref tsubpart18, 505, 202, 32, 16, 1);
        this.ss = "Sort of command that is on the selected line";
        if (this.game.Data.EventObj[this.detail1].CheckMode == 0)
          str1 = "No Check";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 1)
          str1 = "Round Check";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 2)
          str1 = "Early Start Turn (no unter)";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 3)
          str1 = "Late Start Turn Check";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 4)
          str1 = "Late Open Turn Check";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 5)
          str1 = "Close Turn Check";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 6)
          str1 = "After AI Init Check";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 7)
          str1 = "Game Startup Always";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 8)
          str1 = "Early Open Turn Check (depreciated)";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 9)
          str1 = "Load Game Early (by button)";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 10)
          str1 = "Load Game Late";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 11)
          str1 = "Mid Turn (no unter)";
        else if (this.game.Data.EventObj[this.detail1].CheckMode == 12)
          str1 = "Early Cinematics Check";
        this.ss = "Click to change if the event should be round or turn checked or never at all";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.TypMode = this.AddSubPart(ref tsubpart18, 610, 202, 32, 16, 1);
        tsubpart18 =  TextPartClass::new(str1, Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 150, 16, false, tDescript: this.ss);
        this.TypModeB = this.AddSubPart(ref tsubpart18, 650, 202, 150, 16, 0);
        this.ss = "Click to insert a specified number of lines from a specified event below this position.";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONPASTE, tDescript: this.ss);
        this.InsertId = this.AddSubPart(ref tsubpart18, 850, 202, 32, 16, 1);
        this.ss = "Is event executable in editor?";
        if (this.game.Data.EventObj[this.detail1].AllowExecute)
        {
          tsubpart18 =  ButtonPartClass::new(this.game.BUTTONFLAGGED, tDescript: this.ss);
          this.allowExecuteId = this.AddSubPart(ref tsubpart18, 900, 202, 32, 16, 1);
        }
        else
        {
          tsubpart18 =  ButtonPartClass::new(this.game.BUTTONUNFLAGGED, tDescript: this.ss);
          this.allowExecuteId = this.AddSubPart(ref tsubpart18, 900, 202, 32, 16, 1);
        }
        this.ss = "Set Priority. Default is 0. Higher will be executed before, lower after. Current prio = " + this.game.Data.EventObj[this.detail1].Priority.ToString();
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.setPrio = this.AddSubPart(ref tsubpart18, 950, 202, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("Prio=" + this.game.Data.EventObj[this.detail1].Priority.ToString(), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 70, 16, false, tDescript: this.ss);
        this.setPrioB = this.AddSubPart(ref tsubpart18, 990, 202, 70, 16, 0);
        y += 270;
        tsubpart18 =  TextPartClass::new("Type: " + this.COMMANDTYPE[this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type], Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.CurTyp = this.AddSubPart(ref tsubpart18, 10, y, 200, 20, 0);
        this.ss = "Click to change the selected line in a CHECK line. CHECK is used to compare one value to another";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Typ1 = this.AddSubPart(ref tsubpart18, 200, y, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("CHECK", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 50, 16, false, tDescript: this.ss);
        this.Typ1b = this.AddSubPart(ref tsubpart18, 235, y, 50, 16, 0);
        this.ss = "Click to change the selected line in an END CHECK line. END CHECK closes of the code following a CHECK.";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Typ2 = this.AddSubPart(ref tsubpart18, 285, y, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("END.CH", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 50, 16, false, tDescript: this.ss);
        this.Typ2b = this.AddSubPart(ref tsubpart18, 320, y, 50, 16, 0);
        this.ss = "Click to change the selected line in a EXEC line. EXEC lines are there to execute a specific hardcoded function";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Typ3 = this.AddSubPart(ref tsubpart18, 370, y, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("EXEC", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 50, 16, false, tDescript: this.ss);
        this.Typ3b = this.AddSubPart(ref tsubpart18, 405, y, 50, 16, 0);
        this.ss = "Click to change the selected line in a SETVAR line. SETVAR is used to set a variable to another value.";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Typ4 = this.AddSubPart(ref tsubpart18, 455, y, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("SETVAR", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 50, 16, false, tDescript: this.ss);
        this.Typ4b = this.AddSubPart(ref tsubpart18, 490, y, 50, 16, 0);
        this.ss = "Click to change the selected line in a LOOP line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Typ5 = this.AddSubPart(ref tsubpart18, 540, y, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("LOOP", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 50, 16, false, tDescript: this.ss);
        this.Typ5b = this.AddSubPart(ref tsubpart18, 575, y, 50, 16, 0);
        this.ss = "Click to change the selected line in a ENDLOOP line. ENDLOOP is used to close of code following a LOOP line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Typ6 = this.AddSubPart(ref tsubpart18, 625, y, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("END.LP", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 50, 16, false, tDescript: this.ss);
        this.Typ6b = this.AddSubPart(ref tsubpart18, 660, y, 50, 16, 0);
        this.ss = "Click to change the selected line in a COMMENT line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Typ7 = this.AddSubPart(ref tsubpart18, 710, y, 32, 16, 1);
        tsubpart18 =  TextPartClass::new("COMMENT", Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 80, 16, false, tDescript: this.ss);
        this.Typ7b = this.AddSubPart(ref tsubpart18, 745, y, 80, 16, 0);
        if (this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type == 3 | this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type == 7 && this.game.Data.ExecTypeString[Conversions.ToInteger(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 1])] > 0 | this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type == 7)
        {
          this.ss = "Click to change the text of the datastring. Used by for example: messages exec. Or the comment text.";
          tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.typstr = this.AddSubPart(ref tsubpart18, 10, y + 70, 32, 16, 1);
          tsubpart18 =  TextPartClass::new(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString, Font::new("Times New Roman", 10f, FontStyle.Bold, GraphicsUnit.Pixel), 700, 16, false, tDescript: this.ss);
          this.typstrb = this.AddSubPart(ref tsubpart18, 50, y + 70, 700, 16, 0);
        }
      }
      this.detail3 = -1;
      this.detail4 = -1;
      this.detailchoice = -1;
      if (this.detail2 <= -1 || this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type <= 0)
        return;
      let mut type: i32 =  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type;
      this.SetitemName(type);
      if (Strings.Len(this.ITEMNAME[1]) > 0)
      {
        this.ss = "Click to set the first item of this code line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Item1 = this.AddSubPart(ref tsubpart18, 10, y + 40, 32, 16, 1);
        tsubpart18 =  TextPartClass::new(this.ITEMNAME[1], Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 16, false, tDescript: this.ss);
        this.Item1b = this.AddSubPart(ref tsubpart18, 50, y + 40, 350, 16, 0);
        if (type == 1 | type == 3)
        {
          let mut integer1: i32 =  Conversions.ToInteger(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 0]);
          let mut integer2: i32 =  Conversions.ToInteger(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 1]);
          switch (integer1)
          {
            case 2:
              if (integer2 > 0)
              {
                let mut num7: i32 =  this.game.Data.CheckTypeVarCount[integer2];
                for (let mut index5: i32 =  1; index5 <= num7; index5 += 1)
                {
                  this.ss = this.game.Data.CheckTypeVarName[integer2, index5];
                  if (index5 == 1)
                  {
                    tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
                    this.quick1 = this.AddSubPart(ref tsubpart18, 400, y + 40, 32, 16, 1);
                  }
                  if (index5 == 2)
                  {
                    tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
                    this.quick2 = this.AddSubPart(ref tsubpart18, 450, y + 40, 32, 16, 1);
                  }
                  if (index5 == 3)
                  {
                    tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
                    this.quick3 = this.AddSubPart(ref tsubpart18, 500, y + 40, 32, 16, 1);
                  }
                  if (index5 == 4)
                  {
                    tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
                    this.quick4 = this.AddSubPart(ref tsubpart18, 550, y + 40, 32, 16, 1);
                  }
                }
                break;
              }
              break;
            case 4:
              if (integer2 > 0)
              {
                let mut num8: i32 =  this.game.Data.ExecTypeVarCount[integer2];
                for (let mut index6: i32 =  1; index6 <= num8; index6 += 1)
                {
                  this.ss = this.game.Data.ExecTypeVarName[integer2, index6];
                  if (index6 == 1)
                  {
                    tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
                    this.quick1 = this.AddSubPart(ref tsubpart18, 400, y + 40, 32, 16, 1);
                  }
                  if (index6 == 2)
                  {
                    tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
                    this.quick2 = this.AddSubPart(ref tsubpart18, 450, y + 40, 32, 16, 1);
                  }
                  if (index6 == 3)
                  {
                    tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
                    this.quick3 = this.AddSubPart(ref tsubpart18, 500, y + 40, 32, 16, 1);
                  }
                  if (index6 == 4)
                  {
                    tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
                    this.quick4 = this.AddSubPart(ref tsubpart18, 550, y + 40, 32, 16, 1);
                  }
                }
                break;
              }
              break;
          }
        }
      }
      if (Strings.Len(this.ITEMNAME[2]) > 0)
      {
        this.ss = "Click to set the second item of this code line";
        tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.Item2 = this.AddSubPart(ref tsubpart18, 10, y + 70, 32, 16, 1);
        tsubpart18 =  TextPartClass::new(this.ITEMNAME[2], Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 650, 16, false, tDescript: this.ss);
        this.Item2b = this.AddSubPart(ref tsubpart18, 50, y + 70, 650, 16, 0);
      }
      if (Strings.Len(this.ITEMNAME[3]) <= 0)
        return;
      this.ss = "Click to set the third item of this code line";
      tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Item3 = this.AddSubPart(ref tsubpart18, 10, y + 100, 32, 16, 1);
      tsubpart18 =  TextPartClass::new(this.ITEMNAME[3], Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 16, false, tDescript: this.ss);
      this.Item3b = this.AddSubPart(ref tsubpart18, 50, y + 100, 350, 16, 0);
      if (!(type == 1 | type == 4))
        return;
      let mut integer3: i32 =  Conversions.ToInteger(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[2, 0]);
      let mut integer4: i32 =  Conversions.ToInteger(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[2, 1]);
      switch (integer3)
      {
        case 2:
          if (integer4 <= 0)
            break;
          let mut num9: i32 =  this.game.Data.CheckTypeVarCount[integer4];
          for (let mut index7: i32 =  1; index7 <= num9; index7 += 1)
          {
            this.ss = this.game.Data.CheckTypeVarName[integer4, index7];
            if (index7 == 1)
            {
              tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
              this.quick5 = this.AddSubPart(ref tsubpart18, 400, y + 100, 32, 16, 1);
            }
            if (index7 == 2)
            {
              tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
              this.quick6 = this.AddSubPart(ref tsubpart18, 450, y + 100, 32, 16, 1);
            }
            if (index7 == 3)
            {
              tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
              this.quick7 = this.AddSubPart(ref tsubpart18, 500, y + 100, 32, 16, 1);
            }
            if (index7 == 4)
            {
              tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
              this.quick8 = this.AddSubPart(ref tsubpart18, 550, y + 100, 32, 16, 1);
            }
          }
          break;
        case 4:
          if (integer4 <= 0)
            break;
          let mut num10: i32 =  this.game.Data.ExecTypeVarCount[integer4];
          for (let mut index8: i32 =  1; index8 <= num10; index8 += 1)
          {
            this.ss = this.game.Data.ExecTypeVarName[integer4, index8];
            if (index8 == 1)
            {
              tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
              this.quick5 = this.AddSubPart(ref tsubpart18, 400, y + 100, 32, 16, 1);
            }
            if (index8 == 2)
            {
              tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
              this.quick6 = this.AddSubPart(ref tsubpart18, 450, y + 100, 32, 16, 1);
            }
            if (index8 == 3)
            {
              tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
              this.quick7 = this.AddSubPart(ref tsubpart18, 500, y + 100, 32, 16, 1);
            }
            if (index8 == 4)
            {
              tsubpart18 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
              this.quick8 = this.AddSubPart(ref tsubpart18, 550, y + 100, 32, 16, 1);
            }
          }
          break;
      }
    }

    pub fn TabsheetSlots()
    {
      let mut type: i32 =  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type;
      this.String1 = "";
      this.String2 = "";
      this.String3 = "";
      let mut num: i32 =  -1;
      CommandClass command = this.game.Data.EventObj[this.detail1].CommandList[this.detail2];
      if (type == 1)
      {
        if (this.SetSlot == 0 | this.SetSlot == 2)
        {
          if (this.StepCurrent == 0)
          {
            this.String1 = "Check With What?";
            num = 0;
          }
          if (this.StepCurrent == 1)
          {
            if (Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 1.0)
            {
              this.String1 = "Set VarInfo";
              num = 1;
              this.VarExtra = 0;
            }
            if (Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 2.0)
            {
              this.String1 = "Set CheckType";
              num = 2;
            }
          }
          if (this.StepCurrent > 1 & Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 2.0 && this.game.Data.CheckTypeVarCount[Conversions.ToInteger(command.Data[this.SetSlot, 1])] >= this.StepCurrent - 1)
          {
            this.String1 = "Set CheckType Variable " + Conversion.Str( (this.StepCurrent - 1)) + ": '" + this.game.Data.CheckTypeVarName[Conversions.ToInteger(command.Data[this.SetSlot, 1]), this.StepCurrent - 1] + "'";
            num = 1;
            this.VarExtra = (this.StepCurrent - 1) * 3;
          }
        }
        if (this.SetSlot == 1 && this.StepCurrent == 0)
        {
          command.Data[this.SetSlot, 0] = Conversions.ToString(3);
          this.String1 = "Set Conditional";
          num = 3;
        }
      }
      if (type == 3)
      {
        if (this.StepCurrent == 0)
        {
          command.Data[this.SetSlot, 0] = Conversions.ToString(4);
          this.String1 = "Set ExecType";
          num = 4;
        }
        if (this.StepCurrent > 0 & Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 4.0)
        {
          if (this.game.Data.ExecTypeVarCount[Conversions.ToInteger(command.Data[this.SetSlot, 1])] >= this.StepCurrent)
          {
            this.String1 = "Set Exec Type Variable " + Conversion.Str( this.StepCurrent) + ": '" + this.game.Data.ExecTypeVarName[Conversions.ToInteger(command.Data[this.SetSlot, 1]), this.StepCurrent] + "'";
            num = 1;
            this.VarExtra = this.StepCurrent * 3;
          }
          else if (this.game.Data.ExecTypeVarCount[Conversions.ToInteger(command.Data[this.SetSlot, 1])] + 1 == this.StepCurrent)
          {
            if (this.game.Data.ExecTypeString[Conversions.ToInteger(command.Data[this.SetSlot, 1])] == 1)
            {
              this.String1 = "Set Message";
              num = 6;
            }
            if (this.game.Data.ExecTypeString[Conversions.ToInteger(command.Data[this.SetSlot, 1])] == 2)
              this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString = Interaction.InputBox("And give a string (name probably)", "Shadow Empire : Planetary Conquest");
          }
        }
      }
      if (type == 4)
      {
        if (this.SetSlot == 0 && this.StepCurrent == 0)
        {
          this.String1 = "Set VarInfo";
          command.Data[this.SetSlot, 0] = Conversions.ToString(1);
          num = 1;
          this.VarExtra = 0;
        }
        if (this.SetSlot == 2)
        {
          if (this.StepCurrent == 0)
          {
            this.String1 = "Check With What?";
            num = 0;
          }
          if (this.StepCurrent == 1)
          {
            if (Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 1.0)
            {
              this.String1 = "Set VarInfo";
              num = 1;
              this.VarExtra = 0;
            }
            if (Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 2.0)
            {
              this.String1 = "Set CheckType";
              num = 2;
            }
          }
          if (this.StepCurrent > 1 & Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 2.0 && this.game.Data.CheckTypeVarCount[Conversions.ToInteger(command.Data[this.SetSlot, 1])] >= this.StepCurrent - 1)
          {
            this.String1 = "Set CheckType Variable " + Conversion.Str( (this.StepCurrent - 1)) + ": '" + this.game.Data.CheckTypeVarName[Conversions.ToInteger(command.Data[this.SetSlot, 1]), this.StepCurrent - 1] + "'";
            num = 1;
            this.VarExtra = (this.StepCurrent - 1) * 3;
          }
        }
        if (this.SetSlot == 1 && this.StepCurrent == 0)
        {
          command.Data[this.SetSlot, 0] = Conversions.ToString(5);
          this.String1 = "Set SetType";
          num = 5;
        }
      }
      if (type == 5 && this.SetSlot == 1 | this.SetSlot == 2)
      {
        if (this.StepCurrent == 0)
        {
          this.String1 = "Use what Sort of Number?";
          num = 0;
        }
        if (this.StepCurrent == 1)
        {
          if (Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 1.0)
          {
            this.String1 = "Set VarInfo";
            num = 1;
            this.VarExtra = 0;
          }
          if (Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 2.0)
          {
            this.String1 = "Set CheckType";
            num = 2;
          }
        }
        if (this.StepCurrent > 1 & Conversions.ToDouble(command.Data[this.SetSlot, 0]) == 2.0 && this.game.Data.CheckTypeVarCount[Conversions.ToInteger(command.Data[this.SetSlot, 1])] >= this.StepCurrent - 1)
        {
          this.String1 = "Set CheckType Variable " + Conversion.Str( (this.StepCurrent - 1)) + ": '" + this.game.Data.CheckTypeVarName[Conversions.ToInteger(command.Data[this.SetSlot, 1]), this.StepCurrent - 1] + "'";
          num = 1;
          this.VarExtra = (this.StepCurrent - 1) * 3;
        }
      }
      if (num == -1)
      {
        this.DoingSlots = false;
      }
      else
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new(this.String1, Font::new("Times New Roman", 16f, FontStyle.Bold, GraphicsUnit.Pixel), 650, 20, false);
        this.Info1textId = this.AddSubPart(ref tsubpart, 12, 10, 650, 20, 0);
        if (num == 0)
          this.TabSheetNr0();
        if (num == 1)
          this.TabSheetNr1();
        if (num == 2)
          this.TabSheetNr2();
        if (num == 3)
          this.TabSheetNr3();
        if (num == 4)
          this.TabSheetNr4();
        if (num == 5)
          this.TabSheetNr5();
        if (num != 6)
          return;
        this.TabSheetNr6();
      }
    }

    pub fn TabSheetNr0()
    {
      this.ss = "Click to get a value from a hardcoded function";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab2 = this.AddSubPart(ref tsubpart1, 10, 160, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("CheckType", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.Tab2b = this.AddSubPart(ref tsubpart2, 50, 160, 350, 20, 0);
      this.ss = "Click to select a temporary value. These are only remembered within this specific event. ";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab6 = this.AddSubPart(ref tsubpart3, 10, 180, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("TempVar", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.Tab6b = this.AddSubPart(ref tsubpart4, 50, 180, 350, 20, 0);
      this.ss = "Click to select a regimevar value";
      let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab7 = this.AddSubPart(ref tsubpart5, 10, 200, 32, 16, 1);
      let mut tsubpart6: SubPartClass =  TextPartClass::new("RegimeVar", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.Tab7b = this.AddSubPart(ref tsubpart6, 50, 200, 350, 20, 0);
      this.ss = "Click to select the value of a gamevar";
      let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab8 = this.AddSubPart(ref tsubpart7, 10, 220, 32, 16, 1);
      let mut tsubpart8: SubPartClass =  TextPartClass::new("GameVar", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.Tab8b = this.AddSubPart(ref tsubpart8, 50, 220, 350, 20, 0);
      this.ss = "Click to give a real number.";
      let mut tsubpart9: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab9 = this.AddSubPart(ref tsubpart9, 10, 240, 32, 16, 1);
      let mut tsubpart10: SubPartClass =  TextPartClass::new("Real Number", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.Tab9b = this.AddSubPart(ref tsubpart10, 50, 240, 350, 20, 0);
      this.ss = "Click to give a Historical Unit ID.";
      let mut tsubpart11: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.NTab1 = this.AddSubPart(ref tsubpart11, 10, 260, 32, 16, 1);
      let mut tsubpart12: SubPartClass =  TextPartClass::new("Historical Unit ID", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.NTab1b = this.AddSubPart(ref tsubpart12, 50, 260, 350, 20, 0);
      this.ss = "Click to give a Defined Area ID.";
      let mut tsubpart13: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.NTab2 = this.AddSubPart(ref tsubpart13, 10, 280, 32, 16, 1);
      let mut tsubpart14: SubPartClass =  TextPartClass::new("Area ID", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.NTab2b = this.AddSubPart(ref tsubpart14, 50, 280, 350, 20, 0);
      this.ss = "Clieck to select a temporary string. These are only remembered within this specific event.";
      let mut tsubpart15: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab10 = this.AddSubPart(ref tsubpart15, 10, 300, 32, 16, 1);
      let mut tsubpart16: SubPartClass =  TextPartClass::new("TempString", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.Tab10b = this.AddSubPart(ref tsubpart16, 50, 300, 300, 20, 0);
      this.ss = "Click to give a real string.";
      let mut tsubpart17: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.tab19 = this.AddSubPart(ref tsubpart17, 10, 320, 32, 16, 1);
      let mut tsubpart18: SubPartClass =  TextPartClass::new("Real String", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.tab19b = this.AddSubPart(ref tsubpart18, 50, 320, 300, 20, 0);
      this.ss = "Cancel this";
      let mut tsubpart19: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
      this.tabx1 = this.AddSubPart(ref tsubpart19, 10, 390, 32, 16, 1);
    }

    pub fn TabSheetNr1()
    {
      this.ss = "Click to select a temporary value. These are only remembered within this specific event. ";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab6b = this.AddSubPart(ref tsubpart1, 10, 190, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("TempVar", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.tab16b = this.AddSubPart(ref tsubpart2, 50, 190, 350, 20, 0);
      this.ss = "Click to select a regimevar value";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab7b = this.AddSubPart(ref tsubpart3, 10, 210, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("RegimeVar", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.tab17b = this.AddSubPart(ref tsubpart4, 50, 210, 350, 20, 0);
      this.ss = "Click to select the value of a gamevar";
      let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab8b = this.AddSubPart(ref tsubpart5, 10, 230, 32, 16, 1);
      let mut tsubpart6: SubPartClass =  TextPartClass::new("GameVar", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.tab18b = this.AddSubPart(ref tsubpart6, 50, 230, 350, 20, 0);
      this.ss = "Click to give a real number.";
      let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab9b = this.AddSubPart(ref tsubpart7, 10, 250, 32, 16, 1);
      let mut tsubpart8: SubPartClass =  TextPartClass::new("Real Number", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.tab15b = this.AddSubPart(ref tsubpart8, 50, 250, 350, 20, 0);
      this.ss = "Click to give a Historical Unit ID.";
      let mut tsubpart9: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.ntab1x = this.AddSubPart(ref tsubpart9, 10, 270, 32, 16, 1);
      let mut tsubpart10: SubPartClass =  TextPartClass::new("Historical Unit ID", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.NTab1b = this.AddSubPart(ref tsubpart10, 50, 270, 350, 20, 0);
      this.ss = "Click to give a Defined Area ID.";
      let mut tsubpart11: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.ntab2x = this.AddSubPart(ref tsubpart11, 10, 290, 32, 16, 1);
      let mut tsubpart12: SubPartClass =  TextPartClass::new("Area ID", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.NTab2b = this.AddSubPart(ref tsubpart12, 50, 290, 350, 20, 0);
      this.ss = "Clieck to select a temporary string. These are only remembered within this specific event.";
      let mut tsubpart13: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.Tab10b = this.AddSubPart(ref tsubpart13, 10, 310, 32, 16, 1);
      let mut tsubpart14: SubPartClass =  TextPartClass::new("TempString", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.Tab6 = this.AddSubPart(ref tsubpart14, 50, 310, 300, 20, 0);
      this.ss = "Click to give a real string.";
      let mut tsubpart15: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.tab19b = this.AddSubPart(ref tsubpart15, 10, 330, 32, 16, 1);
      let mut tsubpart16: SubPartClass =  TextPartClass::new("Real String", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
      this.Tab8 = this.AddSubPart(ref tsubpart16, 50, 330, 300, 20, 0);
      if (this.detailchoice == 5)
      {
        this.OptionsList3Obj = ListClass::new();
        let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
        for (let mut tdata: i32 =  0; tdata <= historicalUnitCounter; tdata += 1)
          this.OptionsList3Obj.add(Conversion.Str( this.game.Data.HistoricalUnitObj[tdata].ID) + ") " + this.game.Data.HistoricalUnitObj[tdata].Name, tdata);
        if (this.game.Data.HistoricalUnitCounter < this.detail3)
          this.detail3 = -1;
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, this.detail3);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          ListClass optionsList3Obj = this.OptionsList3Obj;
          let mut tlistsize: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - 500) / 16.0) - 3.0);
          let mut detail3: i32 =  this.detail3;
          let mut game: GameClass = this.game;
          ref local1: Bitmap = ref this.OwnBitmap;
          font: Font =  null;
          ref local2: Font = ref font;
          let mut tsubpart17: SubPartClass =  new ListSubPartClass(optionsList3Obj, tlistsize, 490, detail3, game, true, "Historical Units", tbackbitmap: (ref local1), bbx: 10, bby: 250, overruleFont: (ref local2));
          this.OptionsList3Id = this.AddSubPart(ref tsubpart17, 10, 450, 490, Conversion.Int(this.game.ScreenHeight - 500), 0);
        }
        if (this.detail3 > -1)
        {
          let mut tsubpart18: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
          this.Tab5 = this.AddSubPart(ref tsubpart18, 610, 650, 32, 16, 1);
          let mut tsubpart19: SubPartClass =  TextPartClass::new("Ok. Select this one!", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
          this.Tab5b = this.AddSubPart(ref tsubpart19, 650, 650, 350, 20, 0);
        }
      }
      if (this.detailchoice == 6)
      {
        this.OptionsList3Obj = ListClass::new();
        let mut areaCounter: i32 =  this.game.Data.AreaCounter;
        for (let mut tdata: i32 =  0; tdata <= areaCounter; tdata += 1)
          this.OptionsList3Obj.add(Conversion.Str( this.game.Data.AreaObj[tdata].ID) + ") " + this.game.Data.AreaObj[tdata].Name + " (" + Conversion.Str( this.game.Data.AreaObj[tdata].Slot) + "," + Conversion.Str( this.game.Data.AreaObj[tdata].Code) + ")", tdata);
        if (this.game.Data.AreaCounter < this.detail3)
          this.detail3 = -1;
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, this.detail3);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          ListClass optionsList3Obj = this.OptionsList3Obj;
          let mut tlistsize: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - 500) / 16.0) - 3.0);
          let mut detail3: i32 =  this.detail3;
          let mut game: GameClass = this.game;
          ref local3: Bitmap = ref this.OwnBitmap;
          font: Font =  null;
          ref local4: Font = ref font;
          let mut tsubpart20: SubPartClass =  new ListSubPartClass(optionsList3Obj, tlistsize, 490, detail3, game, true, "Areas", tbackbitmap: (ref local3), bbx: 10, bby: 250, overruleFont: (ref local4));
          this.OptionsList3Id = this.AddSubPart(ref tsubpart20, 10, 450, 490, Conversion.Int(this.game.ScreenHeight - 500), 0);
        }
        if (this.detail3 > -1)
        {
          let mut tsubpart21: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
          this.Tab5 = this.AddSubPart(ref tsubpart21, 610, 650, 32, 16, 1);
          let mut tsubpart22: SubPartClass =  TextPartClass::new("Ok. Select this one!", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
          this.Tab5b = this.AddSubPart(ref tsubpart22, 650, 650, 350, 20, 0);
        }
      }
      if (this.detailchoice == 3)
      {
        this.OptionsList3Obj = ListClass::new();
        let mut index: i32 =  0;
        do
        {
          this.OptionsList3Obj.add(Conversion.Str( index) + ") " + this.game.Data.GameSlotName[index], index);
          index += 1;
        }
        while (index <= 499);
        if (499 < this.detail3)
          this.detail3 = -1;
        if (this.OptionsList3Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, this.detail3);
          this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
        }
        else
        {
          ListClass optionsList3Obj = this.OptionsList3Obj;
          let mut tlistsize: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - 500) / 16.0) - 3.0);
          let mut detail3: i32 =  this.detail3;
          let mut game: GameClass = this.game;
          ref local5: Bitmap = ref this.OwnBitmap;
          font: Font =  null;
          ref local6: Font = ref font;
          let mut tsubpart23: SubPartClass =  new ListSubPartClass(optionsList3Obj, tlistsize, 490, detail3, game, true, "Game Variables", tbackbitmap: (ref local5), bbx: 10, bby: 250, overruleFont: (ref local6));
          this.OptionsList3Id = this.AddSubPart(ref tsubpart23, 10, 450, 490,  Math.Round(Conversion.Int( (this.game.ScreenHeight - 500) / 16.0) * 16.0), 0);
        }
        if (this.detail3 > -1)
        {
          let mut tsubpart24: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
          this.Tab5 = this.AddSubPart(ref tsubpart24, 610, 650, 32, 16, 1);
          let mut tsubpart25: SubPartClass =  TextPartClass::new("Ok. Select this one!", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
          this.Tab5b = this.AddSubPart(ref tsubpart25, 650, 650, 350, 20, 0);
        }
      }
      if (this.detailchoice != 2)
        return;
      this.OptionsList4Obj = ListClass::new();
      if (this.game.Data.RegimeCounter > -1)
      {
        let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
        for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
          this.OptionsList4Obj.add(Conversion.Str( index) + ") " + this.game.Data.RegimeObj[index].Name, index);
      }
      if (this.game.Data.RegimeCounter < this.detail4)
        this.detail4 = -1;
      ListClass optionsList4Obj = this.OptionsList4Obj;
      let mut tlistsize1: i32 =   Math.Round(Conversion.Int( (this.game.ScreenHeight - 650) / 16.0) - 3.0);
      let mut detail4: i32 =  this.detail4;
      let mut game1: GameClass = this.game;
      ref local7: Bitmap = ref this.OwnBitmap;
      font1: Font =  null;
      ref local8: Font = ref font1;
      let mut tsubpart26: SubPartClass =  new ListSubPartClass(optionsList4Obj, tlistsize1, 290, detail4, game1, true, "Regimes", tbackbitmap: (ref local7), bbx: 10, bby: 600, overruleFont: (ref local8));
      this.OptionsList4Id = this.AddSubPart(ref tsubpart26, 10, 600, 290,  Math.Round(Conversion.Int( (this.game.ScreenHeight - 650) / 16.0) * 16.0), 0);
      if (this.detail4 <= -1)
        return;
      this.OptionsList3Obj = ListClass::new();
      let mut index1: i32 =  0;
      do
      {
        this.OptionsList3Obj.add(Conversion.Str( index1) + ") " + this.game.Data.RegimeSlotName[index1], index1);
        index1 += 1;
      }
      while (index1 <= 499);
      if (499 < this.detail3)
        this.detail3 = -1;
      if (this.OptionsList3Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, this.detail3);
        this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
      }
      else
      {
        ListClass optionsList3Obj = this.OptionsList3Obj;
        let mut detail3: i32 =  this.detail3;
        let mut game2: GameClass = this.game;
        ref local9: Bitmap = ref this.OwnBitmap;
        font2: Font =  null;
        ref local10: Font = ref font2;
        let mut tsubpart27: SubPartClass =  new ListSubPartClass(optionsList3Obj, 33, 490, detail3, game2, true, "Regime Variables", tbackbitmap: (ref local9), bbx: 10, bby: 250, overruleFont: (ref local10));
        this.OptionsList3Id = this.AddSubPart(ref tsubpart27, 510, 10, 490, 480, 0);
      }
      if (this.detail3 <= -1)
        return;
      let mut tsubpart28: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.Tab5 = this.AddSubPart(ref tsubpart28, 320, 650, 32, 16, 1);
      let mut tsubpart29: SubPartClass =  TextPartClass::new("Ok. Select this one!", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 650, false);
      this.Tab5b = this.AddSubPart(ref tsubpart29, 350, 650, 350, 20, 0);
    }

    pub fn TabSheetNr2()
    {
      this.GroupListObj = ListClass::new();
      if (Operators.ConditionalCompareObjectLess(this.groupdetail,  1, false))
        this.groupdetail =  1;
      let mut tdata1: i32 =  1;
      do
      {
        this.GroupListObj.add(this.game.Data.CheckCategoryName[tdata1], tdata1);
        tdata1 += 1;
      }
      while (tdata1 <= 12);
      let mut tlistselect1: i32 =  -1;
      if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectGreater(this.groupdetail,  0, false), Operators.CompareObjectGreaterEqual( 12, this.groupdetail, false))))
        tlistselect1 = Conversions.ToInteger(Operators.SubtractObject(this.groupdetail,  1));
      if (this.GroupListId > 0)
      {
        this.SubPartList[this.SubpartNr(this.GroupListId)].Refresh(this.GroupListObj, tlistselect1);
        this.SubPartFlag[this.SubpartNr(this.GroupListId)] = true;
      }
      else
      {
        ListClass groupListObj = this.GroupListObj;
        let mut tlistselect2: i32 =  tlistselect1;
        let mut game: GameClass = this.game;
        ref local1: Bitmap = ref this.OwnBitmap;
        font: Font =  null;
        ref local2: Font = ref font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(groupListObj, 24, 290, tlistselect2, game, true, "Check Categories", tbackbitmap: (ref local1), bbx: 10, bby: 50, overruleFont: (ref local2));
        this.GroupListId = this.AddSubPart(ref tsubpart, 10, 50, 290, 432, 0);
      }
      this.OptionsList3Obj = ListClass::new();
      let mut checkTypeCount: i32 =  this.game.Data.CheckTypeCount;
      for (let mut tdata2: i32 =  1; tdata2 <= checkTypeCount; tdata2 += 1)
      {
        if (Conversions.ToBoolean(Operators.OrObject(Operators.OrObject(Operators.CompareObjectEqual( this.game.Data.CheckCategory[tdata2], this.groupdetail, false), Operators.CompareObjectEqual( this.game.Data.CheckCategory2[tdata2], this.groupdetail, false)), Operators.CompareObjectEqual(this.groupdetail,  -1, false))))
          this.OptionsList3Obj.add(this.game.Data.CheckTypeNames[tdata2], tdata2);
      }
      this.OptionsList3Obj.Sort();
      if (this.game.Data.CheckTypeCount < this.detail3)
        this.detail3 = -1;
      let mut tlistselect3: i32 =  -1;
      let mut listCount: i32 =  this.OptionsList3Obj.ListCount;
      for (let mut index: i32 =  0; index <= listCount; index += 1)
      {
        if (this.detail3 == this.OptionsList3Obj.ListData[index])
          tlistselect3 = index;
      }
      if (this.OptionsList3Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, tlistselect3);
        this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
      }
      else
      {
        ListClass optionsList3Obj = this.OptionsList3Obj;
        let mut tlistselect4: i32 =  tlistselect3;
        let mut game: GameClass = this.game;
        ref local3: Bitmap = ref this.OwnBitmap;
        font: Font =  null;
        ref local4: Font = ref font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList3Obj, 30, 290, tlistselect4, game, true, "Hardcoded check functions", tbackbitmap: (ref local3), bbx: 310, bby: 50, overruleFont: (ref local4));
        this.OptionsList3Id = this.AddSubPart(ref tsubpart, 310, 50, 290, 528, 0);
      }
      if (this.detail3 > -1)
      {
        let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
        this.Tab3 = this.AddSubPart(ref tsubpart1, 710, 50, 32, 16, 1);
        let mut tsubpart2: SubPartClass =  TextPartClass::new("Ok. Select this one!", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
        this.Tab3b = this.AddSubPart(ref tsubpart2, 650, 150, 350, 20, 0);
      }
      str: String = "";
      if (this.detail3 > -1)
      {
        let mut num: i32 =  this.game.Data.CheckTypeVarCount[this.detail3];
        for (let mut Number: i32 =  1; Number <= num; Number += 1)
          str = str + Conversion.Str( Number) + ") " + this.game.Data.CheckTypeVarName[this.detail3, Number] + "\r\n";
        tText: String = str + "\r\n" + this.game.Data.CheckDesc[this.detail3];
        if (this.DescriptId > 0)
          this.RemoveSubPart(this.DescriptId);
        let mut tsubpart3: SubPartClass =  new TextAreaClass(this.game, 290, 15, Font::new(this.game.FontCol.Families[1], 13f, FontStyle.Regular, GraphicsUnit.Pixel), "Description", true, tText, Color.White, tbackbitmap: (ref this.OwnBitmap), bbx: 10, bby: 470);
        this.DescriptId = this.AddSubPart(ref tsubpart3, 10, 470, 290, 2528, 0);
        let mut tsubpart4: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
        this.Tab4 = this.AddSubPart(ref tsubpart4, 710, 50, 32, 16, 1);
        let mut tsubpart5: SubPartClass =  TextPartClass::new("Select", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 50, 20, false);
        this.Tab4b = this.AddSubPart(ref tsubpart5, 750, 50, 50, 20, 0);
      }
      else if (this.DescriptId > 0)
        this.RemoveSubPart(this.DescriptId);
      this.ss = "Cancel this";
      let mut tsubpart6: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
      this.tabx2 = this.AddSubPart(ref tsubpart6, 710, 100, 32, 16, 1);
    }

    pub fn TabSheetNr3()
    {
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab11 = this.AddSubPart(ref tsubpart1, 10, 130, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("> IS BIGGER", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab11b = this.AddSubPart(ref tsubpart2, 50, 130, 350, 20, 0);
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab12 = this.AddSubPart(ref tsubpart3, 10, 160, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("< IS SMALLER", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab12b = this.AddSubPart(ref tsubpart4, 50, 160, 350, 20, 0);
      let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab13 = this.AddSubPart(ref tsubpart5, 10, 190, 32, 16, 1);
      let mut tsubpart6: SubPartClass =  TextPartClass::new("== IS EQUAL", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab13b = this.AddSubPart(ref tsubpart6, 50, 190, 350, 20, 0);
      let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab131 = this.AddSubPart(ref tsubpart7, 10, 220, 32, 16, 1);
      let mut tsubpart8: SubPartClass =  TextPartClass::new("=> IS BIGGER OR EQUAL", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab131b = this.AddSubPart(ref tsubpart8, 50, 220, 350, 20, 0);
      let mut tsubpart9: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab132 = this.AddSubPart(ref tsubpart9, 10, 250, 32, 16, 1);
      let mut tsubpart10: SubPartClass =  TextPartClass::new("=< IS SMALLER OR EQUAL", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab132b = this.AddSubPart(ref tsubpart10, 50, 250, 350, 20, 0);
      let mut tsubpart11: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab133 = this.AddSubPart(ref tsubpart11, 10, 280, 32, 16, 1);
      let mut tsubpart12: SubPartClass =  TextPartClass::new("!= IS NOT EQUAL", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab133b = this.AddSubPart(ref tsubpart12, 50, 280, 350, 20, 0);
    }

    pub fn TabSheetNr4()
    {
      if (Operators.ConditionalCompareObjectLess(this.groupdetail,  1, false))
        this.groupdetail =  1;
      this.GroupListObj = ListClass::new();
      let mut tdata1: i32 =  1;
      do
      {
        this.GroupListObj.add(this.game.Data.ExecCategoryName[tdata1], tdata1);
        tdata1 += 1;
      }
      while (tdata1 <= 22);
      let mut tlistselect1: i32 =  -1;
      if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectGreater(this.groupdetail,  0, false), Operators.CompareObjectGreaterEqual( 22, this.groupdetail, false))))
        tlistselect1 = Conversions.ToInteger(Operators.SubtractObject(this.groupdetail,  1));
      if (this.GroupListId > 0)
      {
        this.SubPartList[this.SubpartNr(this.GroupListId)].Refresh(this.GroupListObj, tlistselect1);
        this.SubPartFlag[this.SubpartNr(this.GroupListId)] = true;
      }
      else
      {
        ListClass groupListObj = this.GroupListObj;
        let mut tlistselect2: i32 =  tlistselect1;
        let mut game: GameClass = this.game;
        ref local1: Bitmap = ref this.OwnBitmap;
        font: Font =  null;
        ref local2: Font = ref font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(groupListObj, 26, 290, tlistselect2, game, true, "Exec Categories", tbackbitmap: (ref local1), bbx: 10, bby: 50, overruleFont: (ref local2));
        this.GroupListId = this.AddSubPart(ref tsubpart, 10, 50, 290, 464, 0);
      }
      this.OptionsList3Obj = ListClass::new();
      let mut execTypeCount: i32 =  this.game.Data.ExecTypeCount;
      for (let mut tdata2: i32 =  1; tdata2 <= execTypeCount; tdata2 += 1)
      {
        if (Conversions.ToBoolean(Operators.OrObject(Operators.OrObject(Operators.CompareObjectEqual( this.game.Data.ExecCategory[tdata2], this.groupdetail, false), Operators.CompareObjectEqual( this.game.Data.ExecCategory2[tdata2], this.groupdetail, false)), Operators.CompareObjectEqual(this.groupdetail,  -1, false))))
          this.OptionsList3Obj.add(this.game.Data.ExecTypeNames[tdata2], tdata2);
      }
      this.OptionsList3Obj.Sort();
      if (this.game.Data.ExecTypeCount < this.detail3)
        this.detail3 = -1;
      let mut tlistselect3: i32 =  -1;
      let mut listCount: i32 =  this.OptionsList3Obj.ListCount;
      for (let mut index: i32 =  0; index <= listCount; index += 1)
      {
        if (this.detail3 == this.OptionsList3Obj.ListData[index])
          tlistselect3 = index;
      }
      if (this.OptionsList3Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, tlistselect3);
        this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
      }
      else
      {
        ListClass optionsList3Obj = this.OptionsList3Obj;
        let mut tlistselect4: i32 =  tlistselect3;
        let mut game: GameClass = this.game;
        ref local3: Bitmap = ref this.OwnBitmap;
        font: Font =  null;
        ref local4: Font = ref font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList3Obj, 35, 290, tlistselect4, game, true, "Hardcoded exec functions", tbackbitmap: (ref local3), bbx: 310, bby: 50, overruleFont: (ref local4));
        this.OptionsList3Id = this.AddSubPart(ref tsubpart, 310, 50, 290, 608, 0);
      }
      str: String = "";
      if (this.detail3 > -1)
      {
        let mut num: i32 =  this.game.Data.ExecTypeVarCount[this.detail3];
        for (let mut Number: i32 =  1; Number <= num; Number += 1)
          str = str + Conversion.Str( Number) + ") " + this.game.Data.ExecTypeVarName[this.detail3, Number] + "\r\n";
        if (this.game.Data.ExecTypeString[this.detail3] == 2)
          str += "+ You can input a string\r\n";
        if (this.game.Data.ExecTypeString[this.detail3] == 1)
          str += "+ You can input a textfield\r\n";
        tText: String = str + "\r\n" + this.game.Data.ExecDesc[this.detail3];
        if (this.DescriptId > 0)
          this.RemoveSubPart(this.DescriptId);
        let mut tsubpart1: SubPartClass =  new TextAreaClass(this.game, 290, 12, Font::new(this.game.FontCol.Families[1], 13f, FontStyle.Regular, GraphicsUnit.Pixel), "Description", true, tText, Color.White, tbackbitmap: (ref this.OwnBitmap), bbx: 10, bby: 516);
        this.DescriptId = this.AddSubPart(ref tsubpart1, 10, 516, 290, 224, 0);
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
        this.Tab4 = this.AddSubPart(ref tsubpart2, 710, 50, 32, 16, 1);
        let mut tsubpart3: SubPartClass =  TextPartClass::new("Select", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 50, 20, false);
        this.Tab4b = this.AddSubPart(ref tsubpart3, 750, 50, 50, 20, 0);
      }
      else if (this.DescriptId > 0)
        this.RemoveSubPart(this.DescriptId);
      this.ss = "Cancel this";
      let mut tsubpart4: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
      this.tabx1 = this.AddSubPart(ref tsubpart4, 710, 100, 32, 16, 1);
    }

    pub fn TabSheetNr5()
    {
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab14 = this.AddSubPart(ref tsubpart1, 10, 130, 32, 16, 1);
      let mut tsubpart2: SubPartClass =  TextPartClass::new("= SET TO", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab14b = this.AddSubPart(ref tsubpart2, 50, 130, 350, 20, 0);
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab15 = this.AddSubPart(ref tsubpart3, 10, 160, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("+= ADD", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab15b = this.AddSubPart(ref tsubpart4, 50, 160, 350, 20, 0);
      let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab16 = this.AddSubPart(ref tsubpart5, 10, 190, 32, 16, 1);
      let mut tsubpart6: SubPartClass =  TextPartClass::new("-= SUBTRACT", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab16b = this.AddSubPart(ref tsubpart6, 50, 190, 350, 20, 0);
      let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab17 = this.AddSubPart(ref tsubpart7, 10, 220, 32, 16, 1);
      let mut tsubpart8: SubPartClass =  TextPartClass::new("/= DIVIDE", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab17b = this.AddSubPart(ref tsubpart8, 50, 220, 350, 20, 0);
      let mut tsubpart9: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
      this.tab18 = this.AddSubPart(ref tsubpart9, 10, 250, 32, 16, 1);
      let mut tsubpart10: SubPartClass =  TextPartClass::new("*= MULTIPLY", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 350, 20, false);
      this.tab18b = this.AddSubPart(ref tsubpart10, 50, 250, 350, 20, 0);
    }

    pub fn TabSheetNr6()
    {
      SubPartClass tsubpart1;
      if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 1]) == 11.0)
      {
        let mut tsubpart2: SubPartClass =  new TextAreaClass(this.game, 400, 15, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), "Text", true, this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString, Color.White, tbackbitmap: (ref this.OwnBitmap), bbx: 10, bby: 100);
        this.tta = this.AddSubPart(ref tsubpart2, 10, 100, 400, 288, 0);
        let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.OKBALL);
        this.tok2 = this.AddSubPart(ref tsubpart3, 10, 500, 32, 32, 1);
      }
      else
      {
        str1: String = this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString;
        let mut index1: i32 =  -1;
        let mut index2: i32 =  0;
        do
        {
          this.temptext[index2] = "";
          this.tempoption[index2] = 0;
          this.temprefer[index2] = -1;
          index2 += 1;
        }
        while (index2 <= 19);
        let mut num1: i32 =  0;
        if (Strings.Len(str1) > 0)
        {
          do
          {
            let mut Length: i32 =  Strings.InStr(str1, "#", CompareMethod.Text);
            if (Length > 0)
            {
              str2: String = Strings.Left(str1, Length);
              str3: String = Strings.Left(str2, Strings.Len(str2) - 1);
              index1 += 1;
              this.temptext[index1] = str3;
              if (Strings.Len(str1) > Length)
                str1 = Strings.Mid(str1, Length + 1);
              else
                num1 = 1;
            }
            else
            {
              if (Strings.Len(str1) > 0)
              {
                index1 += 1;
                this.temptext[index1] = str1;
              }
              num1 = 1;
            }
          }
          while (num1 == 0);
        }
        let mut num2: i32 =  index1;
        for (let mut index3: i32 =  0; index3 <= num2; index3 += 1)
        {
          let mut num3: i32 =  Strings.InStr(this.temptext[index3], "*", CompareMethod.Text);
          str4: String = Strings.Mid(this.temptext[index3], num3 + 1);
          let mut num4: i32 =  Strings.InStr(str4, "@", CompareMethod.Text);
          if (num3 > 0 & num4 > 0)
          {
            str5: String = Strings.Mid(str4, num4 + 1);
            str4 = Strings.Left(str4, Strings.Len(str4) - (1 + Strings.Len(str5)));
            let mut num5: i32 =   Math.Round(Conversion.Val(str5));
            this.temprefer[index3] = num5;
            this.tempoption[index3] = 1;
            num6: i32;
            num6 += 1;
          }
          this.temptext[index3] = str4;
        }
        let mut num7: i32 =  index1;
        for (let mut index4: i32 =  0; index4 <= num7; index4 += 1)
        {
          let mut y: i32 =  100 + index4 * 25;
          int[] tbutton = this.tbutton;
          let mut index5: i32 =  index4;
          let mut tsubpart4: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK);
          let mut num8: i32 =  this.AddSubPart(ref tsubpart4, 10, y, 32, 16, 1);
          tbutton[index5] = num8;
          int[] tline = this.tline;
          let mut index6: i32 =  index4;
          tsubpart4 =  TextPartClass::new(this.temptext[index4], Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 650, 20, false);
          let mut num9: i32 =  this.AddSubPart(ref tsubpart4, 50, y, 650, 20, 0);
          tline[index6] = num9;
          if (this.tempoption[index4] == 1)
          {
            int[] toption = this.toption;
            let mut index7: i32 =  index4;
            let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONFLAGGED);
            let mut num10: i32 =  this.AddSubPart(ref tsubpart5, 750, y, 32, 16, 1);
            toption[index7] = num10;
            int[] trefer = this.trefer;
            let mut index8: i32 =  index4;
            let mut tsubpart6: SubPartClass =  TextPartClass::new(Conversion.Str( this.temprefer[index4]), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
            let mut num11: i32 =  this.AddSubPart(ref tsubpart6, 790, y, 200, 20, 0);
            trefer[index8] = num11;
          }
          else
          {
            int[] toption = this.toption;
            let mut index9: i32 =  index4;
            let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONUNFLAGGED);
            let mut num12: i32 =  this.AddSubPart(ref tsubpart7, 750, y, 32, 16, 1);
            toption[index9] = num12;
          }
        }
        if (index1 < 19)
        {
          for (let mut index10: i32 =  index1 + 1; index10 <= 19; index10 += 1)
          {
            let mut y: i32 =  100 + index10 * 25;
            int[] tbutton = this.tbutton;
            let mut index11: i32 =  index10;
            tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK);
            let mut num13: i32 =  this.AddSubPart(ref tsubpart1, 10, y, 32, 16, 1);
            tbutton[index11] = num13;
          }
        }
      }
      tsubpart1 =  ButtonPartClass::new(this.game.OKBALL);
      this.tok = this.AddSubPart(ref tsubpart1, 10, 700, 32, 32, 1);
      tsubpart1 =  TextPartClass::new("Ok, Alright!", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
      this.tab14b = this.AddSubPart(ref tsubpart1, 50, 700, 200, 20, 0);
    }

    pub fn SetitemName(typ: i32)
    {
      let mut index: i32 =  1;
      do
      {
        this.ITEMNAME[index] = "";
        index += 1;
      }
      while (index <= 5);
      if (typ == 1)
      {
        this.ITEMNAME[1] = "ITEM1: " + this.ItemInfo(this.detail1, this.detail2, 0);
        this.ITEMNAME[2] = "CONDITION: " + this.ItemInfo(this.detail1, this.detail2, 1);
        this.ITEMNAME[3] = "ITEM2: " + this.ItemInfo(this.detail1, this.detail2, 2);
      }
      if (typ == 3)
        this.ITEMNAME[1] = "EXECTYPE: " + this.ItemInfo(this.detail1, this.detail2, 0);
      if (typ == 4)
      {
        this.ITEMNAME[1] = "SETVAR: " + this.ItemInfo(this.detail1, this.detail2, 0);
        this.ITEMNAME[2] = "SETTYPE: " + this.ItemInfo(this.detail1, this.detail2, 1);
        this.ITEMNAME[3] = "VALUE: " + this.ItemInfo(this.detail1, this.detail2, 2);
      }
      if (typ != 5)
        return;
      this.ITEMNAME[1] = "TEMPVAR: " + this.ItemInfo(this.detail1, this.detail2, 0);
      this.ITEMNAME[2] = "STARTVAL: " + this.ItemInfo(this.detail1, this.detail2, 1);
      this.ITEMNAME[3] = "ENDVAL: " + this.ItemInfo(this.detail1, this.detail2, 2);
    }

    pub ItemInfo: String(enr: i32, cnr: i32, inr: i32)
    {
      CommandClass command = this.game.Data.EventObj[enr].CommandList[cnr];
      if (command.type == 5 && inr == 0)
        return "TempVar" + Strings.Trim(Conversion.Str( command.Data[inr, 0]));
      if (Conversions.ToDouble(command.Data[inr, 0]) == 0.0)
        return "Empty".to_owned();
      if (Conversions.ToDouble(command.Data[inr, 0]) == 1.0)
      {
        if (command.type == 5 & inr == 1)
          return "FROM " + this.GetVarInfo(enr, cnr, inr, 1);
        return command.type == 5 & inr == 2 ? "TO " + this.GetVarInfo(enr, cnr, inr, 1) : this.GetVarInfo(enr, cnr, inr, 1);
      }
      if (Conversions.ToDouble(command.Data[inr, 0]) == 2.0)
      {
        if (Conversions.ToDouble(command.Data[inr, 1]) == 0.0)
          return "Empty CheckType";
        str1: String = this.game.Data.CheckTypeNames[Conversions.ToInteger(command.Data[inr, 1])];
        if (this.game.Data.CheckTypeVarCount[Conversions.ToInteger(command.Data[inr, 1])] > 0)
        {
          str2: String = str1 + "(";
          let mut num: i32 =  this.game.Data.CheckTypeVarCount[Conversions.ToInteger(command.Data[inr, 1])];
          for (let mut index: i32 =  1; index <= num; index += 1)
          {
            if (index > 1)
              str2 += ", ";
            str2 += this.GetVarInfo(enr, cnr, inr, 1 + 3 * index);
          }
          str1 = str2 + ")";
        }
        if (command.type == 5 & inr == 1)
          return "FROM " + str1;
        return command.type == 5 & inr == 2 ? "TO " + str1 : str1;
      }
      if (Conversions.ToDouble(command.Data[inr, 0]) == 3.0)
      {
        if (Conversions.ToDouble(command.Data[inr, 1]) == 0.0)
          return "Empty Condition";
        if (Conversions.ToDouble(command.Data[inr, 1]) == 1.0)
          return ">";
        if (Conversions.ToDouble(command.Data[inr, 1]) == 2.0)
          return "<";
        if (Conversions.ToDouble(command.Data[inr, 1]) == 3.0)
          return "==";
        if (Conversions.ToDouble(command.Data[inr, 1]) == 4.0)
          return "=>";
        if (Conversions.ToDouble(command.Data[inr, 1]) == 5.0)
          return "=<";
        if (Conversions.ToDouble(command.Data[inr, 1]) == 6.0)
          return "!=";
      }
      else
      {
        if (Conversions.ToDouble(command.Data[inr, 0]) == 4.0)
        {
          if (Conversions.ToDouble(command.Data[inr, 1]) == 0.0)
            return "Empty ExecType";
          str3: String = this.game.Data.ExecTypeNames[Conversions.ToInteger(command.Data[inr, 1])];
          if (this.game.Data.ExecTypeVarCount[Conversions.ToInteger(command.Data[inr, 1])] > 0)
          {
            str4: String = str3 + "(";
            let mut num: i32 =  this.game.Data.ExecTypeVarCount[Conversions.ToInteger(command.Data[inr, 1])];
            for (let mut index: i32 =  1; index <= num; index += 1)
            {
              if (index > 1)
                str4 += ", ";
              str4 += this.GetVarInfo(enr, cnr, inr, 1 + 3 * index);
            }
            str3 = str4 + ")";
          }
          return str3;
        }
        if (Conversions.ToDouble(command.Data[inr, 0]) == 5.0)
        {
          if (Conversions.ToDouble(command.Data[inr, 1]) == 0.0)
            return "Empty SetType";
          if (Conversions.ToDouble(command.Data[inr, 1]) == 1.0)
            return "=";
          if (Conversions.ToDouble(command.Data[inr, 1]) == 2.0)
            return "+";
          if (Conversions.ToDouble(command.Data[inr, 1]) == 3.0)
            return "-";
          if (Conversions.ToDouble(command.Data[inr, 1]) == 4.0)
            return "/";
          if (Conversions.ToDouble(command.Data[inr, 1]) == 5.0)
            return "*";
        }
      }
      str: String;
      return str;
    }

    pub GetVarInfo: String(enr: i32, cnr: i32, inr: i32, datnr: i32)
    {
      CommandClass command = this.game.Data.EventObj[enr].CommandList[cnr];
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 0.0)
        return "Empty VarInfo";
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 1.0)
        return "TempVar" + Strings.Trim(Conversion.Str( command.Data[inr, datnr + 1]));
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 2.0)
      {
        varInfo: String;
        if (Conversions.ToDouble(command.Data[inr, datnr + 1]) <=  this.game.Data.RegimeCounter)
          varInfo = this.game.Data.RegimeObj[Conversions.ToInteger(command.Data[inr, datnr + 1])].Name + "_" + this.game.Data.RegimeSlotName[Conversions.ToInteger(command.Data[inr, datnr + 2])] + "(#" + Strings.Trim(Conversion.Str( command.Data[inr, datnr + 2])) + ")";
        else
          varInfo = "Non-existing Regime" + Conversion.Str( command.Data[inr, datnr + 1]) + "_" + this.game.Data.RegimeSlotName[Conversions.ToInteger(command.Data[inr, datnr + 2])] + "(#" + Strings.Trim(Conversion.Str( command.Data[inr, datnr + 2])) + ")";
        return varInfo;
      }
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 3.0)
      {
        str: String = "Gameslot_(#" + Strings.Trim(Conversion.Str( command.Data[inr, datnr + 1])) + ")_" + this.game.Data.GameSlotName[Conversions.ToInteger(command.Data[inr, datnr + 1])];
        if (str.Length > 49)
          str = Strings.Left(str, 49) + "...";
        return str;
      }
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 4.0)
        return Strings.Trim(Conversion.Str( command.Data[inr, datnr + 1]));
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 5.0)
      {
        let mut historicalUnitById: i32 =  this.game.HandyFunctionsObj.GetHistoricalUnitByID(Conversions.ToInteger(command.Data[inr, datnr + 1]));
        return historicalUnitById <= -1 ? "_!!Unit_Aint_OnMap!!_" : "[HisID" + this.game.Data.HistoricalUnitObj[historicalUnitById].ID.ToString() + "]" + this.game.Data.HistoricalUnitObj[historicalUnitById].Name;
      }
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 6.0)
      {
        let mut areaById: i32 =  this.game.HandyFunctionsObj.GetAreaByID(Conversions.ToInteger(command.Data[inr, datnr + 1]));
        return areaById <= -1 ? "_!!Area_Aint_OnMap!!_" : "[Area]" + this.game.Data.AreaObj[areaById].Name;
      }
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 7.0)
        return "TempString" + Strings.Trim(Conversion.Str( command.Data[inr, datnr + 1]));
      if (Conversions.ToDouble(command.Data[inr, datnr]) == 8.0)
        return "'" + command.Data[inr, datnr + 1] + "'";
      varInfo1: String;
      return varInfo1;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
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
            if (num1 == this.ExecNow)
            {
              if (this.detail1 > -1)
              {
                this.game.HandyFunctionsObj.RedimStats();
                let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                for (let mut regnr: i32 =  0; regnr <= regimeCounter; regnr += 1)
                {
                  this.game.HandyFunctionsObj.ClearHistory( regnr);
                  this.game.ProcessingObj.SetInitialReconAndZOC(regnr);
                }
                let mut turn: i32 =  this.game.Data.Turn;
                this.game.Data.Turn = 0;
                this.game.EventRelatedObj.DoCheckSpecificEvent(this.detail1);
                this.game.Data.EventObj[this.detail1].Blocked = false;
                this.game.Data.Turn = turn;
                let mut num2: i32 =   Interaction.MsgBox( "Done");
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == this.Import1)
              {
                str: String = this.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1|SE1 Map file(*.se1map)|*.se1map|SE1 Master file(*.se1master)|*.se1master|SE1 Event library(*.se1evlib)|*.se1evlib|SE1 Troops&Equipment library(*.se1troops)|*.se1troops|SE1 Historical library(*.se1his)|*.se1his|SE1 Officer Card Library(*.se1offcard)|*.se1offcard|SE1 Officer library(*.se1off)|*.se1off", "Pick a scenario to load events from...", this.game.AppPath + "scenarios\\", false);
                if (File.Exists(str))
                {
                  this.game.HandyFunctionsObj.Unzip(str);
                  dataClass: DataClass = DataClass.deserialize(str);
                  this.game.HandyFunctionsObj.ZipFile(str);
                  InputStr: String = Interaction.InputBox("Give Event# to import, -1=all");
                  let mut eventCounter: i32 =  dataClass.EventCounter;
                  for (let mut index2: i32 =  0; index2 <= eventCounter; index2 += 1)
                  {
                    if (Conversion.Val(InputStr) != -1.0 & Conversion.Val(InputStr) ==  index2 & this.detail1 > -1)
                    {
                      if (Interaction.MsgBox( "Overwrite selected event?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                        this.game.Data.EventObj[this.detail1] = dataClass.EventObj[index2].Clone();
                      else if (Operators.CompareString(this.game.Data.EventObj[this.detail1].Name, "New Event", false) == 0)
                      {
                        this.game.Data.EventObj[this.detail1] = dataClass.EventObj[index2].Clone();
                        this += 1.game.Data.EventIdCounter;
                        this.game.Data.EventObj[this.detail1].Id = this.game.Data.EventIdCounter;
                      }
                      else
                      {
                        this.game.Data.AddEvent();
                        this.game.Data.EventObj[this.game.Data.EventCounter] = dataClass.EventObj[index2].Clone();
                        this += 1.game.Data.EventIdCounter;
                        this.game.Data.EventObj[this.game.Data.EventCounter].Id = this.game.Data.EventIdCounter;
                      }
                    }
                    else if ( index2 == Conversion.Val(InputStr) | Conversion.Val(InputStr) == -1.0)
                    {
                      if (this.detail1 > -1 & Conversion.Val(InputStr) != -1.0)
                      {
                        if (Operators.CompareString(this.game.Data.EventObj[this.detail1].Name, "New Event", false) == 0)
                        {
                          this.game.Data.EventObj[this.detail1] = dataClass.EventObj[index2].Clone();
                          this += 1.game.Data.EventIdCounter;
                          this.game.Data.EventObj[this.detail1].Id = this.game.Data.EventIdCounter;
                        }
                        else
                        {
                          this.game.Data.AddEvent();
                          this.game.Data.EventObj[this.game.Data.EventCounter] = dataClass.EventObj[index2].Clone();
                          this += 1.game.Data.EventIdCounter;
                          this.game.Data.EventObj[this.game.Data.EventCounter].Id = this.game.Data.EventIdCounter;
                        }
                      }
                      else
                      {
                        this.game.Data.AddEvent();
                        this.game.Data.EventObj[this.game.Data.EventCounter] = dataClass.EventObj[index2].Clone();
                        this += 1.game.Data.EventIdCounter;
                        this.game.Data.EventObj[this.game.Data.EventCounter].Id = this.game.Data.EventIdCounter;
                      }
                    }
                  }
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Import2)
              {
                str: String = this.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to load events from...", this.game.AppPath + "scenarios\\", false);
                if (File.Exists(str))
                {
                  this.game.HandyFunctionsObj.Unzip(str);
                  dataClass: DataClass = DataClass.deserialize(str);
                  this.game.HandyFunctionsObj.ZipFile(str);
                  InputStr: String = Interaction.InputBox("Give Event Group# to import, -1=all");
                  let mut eventCounter: i32 =  dataClass.EventCounter;
                  for (let mut index3: i32 =  0; index3 <= eventCounter; index3 += 1)
                  {
                    if ( dataClass.EventObj[index3].Category == Conversion.Val(InputStr) | Conversion.Val(InputStr) == -1.0)
                    {
                      this.game.Data.AddEvent();
                      this.game.Data.EventObj[this.game.Data.EventCounter] = dataClass.EventObj[index3].Clone();
                      this += 1.game.Data.EventIdCounter;
                      this.game.Data.EventObj[this.game.Data.EventCounter].Id = this.game.Data.EventIdCounter;
                    }
                  }
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.info1id)
              {
                windowReturnClass.AddCommand(3, 2);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.AddEventId)
              {
                if (this.detail1 == -1)
                  this.detail1 = this.game.Data.EventCounter;
                this.game.Data.AddEvent(this.detail1);
                if (this.cat > -1)
                  this.game.Data.EventObj[this.detail1 + 1].Category = this.cat;
                this += 1.detail1;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.EventCopy)
              {
                this.game.EditObj.TempEvent = this.game.Data.EventObj[this.detail1].Clone();
                return windowReturnClass;
              }
              if (num1 == this.EventPaste)
              {
                if (this.game.EditObj.TempEvent != null)
                {
                  this.game.Data.AddEvent(this.detail1);
                  let mut id: i32 =  this.game.Data.EventObj[this.detail1 + 1].Id;
                  this += 1.detail1;
                  this.game.Data.EventObj[this.detail1] = this.game.EditObj.TempEvent.Clone();
                  this.game.Data.EventObj[this.detail1].Id = id;
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.replaceId)
              {
                let mut num3: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("What kind of replacement?" + "\r\n0) Cancel" + "\r\n1) RealString, EXACT" + "\r\n2) RealString, PARTIAL + EXACT", "Replace Options")));
                if (!(num3 >= 1 & num3 <= 2))
                  return windowReturnClass;
                let mut num4: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Extend of replacement" + "\r\n0) Cancel" + "\r\n1) Only in selected event" + "\r\n2) Only in selected library (or all without library)" + "\r\n3) In all events", "Replace Options")));
                if (!(num4 >= 1 & num4 <= 3))
                  return windowReturnClass;
                if (this.detail1 == -1 & num4 < 3)
                {
                  let mut num5: i32 =   Interaction.MsgBox( "You must have an event selected to use the replace function like this.", Title: ( "Shadow Empire : Planetary Conquest"));
                  return windowReturnClass;
                }
                str1: String = Interaction.InputBox("Search for");
                newValue: String = Interaction.InputBox("Replace with");
                let mut eventCounter: i32 =  this.game.Data.EventCounter;
                num6: i32;
                num7: i32;
                str2: String;
                for (let mut index4: i32 =  0; index4 <= eventCounter; index4 += 1)
                {
                  bool flag1 = false;
                  bool flag2 = true;
                  if (num4 == 1 && index4 != this.detail1)
                    flag2 = false;
                  if (num4 == 2 && this.game.Data.EventObj[index4].LibId.libSlot != this.game.Data.EventObj[this.detail1].LibId.libSlot)
                    flag2 = false;
                  if (flag2)
                  {
                    let mut commandCounter: i32 =  this.game.Data.EventObj[index4].CommandCounter;
                    for (let mut index5: i32 =  0; index5 <= commandCounter; index5 += 1)
                    {
                      let mut index6: i32 =  0;
                      do
                      {
                        let mut num8: i32 =  0;
                        do
                        {
                          let mut integer: i32 =  Conversions.ToInteger(this.game.Data.EventObj[index4].CommandList[index5].Data[index6, 1 + num8 * 3]);
                          str3: String = this.game.Data.EventObj[index4].CommandList[index5].Data[index6, 1 + num8 * 3 + 1];
                          if (num3 == 1 & integer == 8)
                          {
                            if (Operators.CompareString(str3, str1, false) == 0)
                            {
                              this.game.Data.EventObj[index4].CommandList[index5].Data[index6, 1 + num8 * 3 + 1] = newValue;
                              num6 += 1;
                              flag1 = true;
                            }
                          }
                          else if (num3 == 2 & integer == 8 && Strings.InStr(str3, str1) > 0)
                          {
                            str4: String = str3.Replace(str1, newValue);
                            this.game.Data.EventObj[index4].CommandList[index5].Data[index6, 1 + num8 * 3 + 1] = str4;
                            num6 += 1;
                            flag1 = true;
                          }
                          num8 += 1;
                        }
                        while (num8 <= 3);
                        index6 += 1;
                      }
                      while (index6 <= 2);
                    }
                  }
                  if (flag1)
                  {
                    num7 += 1;
                    if (num7 > 1)
                      str2 += ", ";
                    str2 = str2 + "[" + index4.ToString() + "] " + this.game.Data.EventObj[index4].Name;
                  }
                }
                let mut num9: i32 =   Interaction.MsgBox( ("Made " + num6.ToString() + " changes in " + num7.ToString() + " events. " + str2), Title: ( "Shadow Empire : Planetary Conquest"));
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.setCat)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 32, this.detail1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.setLib)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 90, this.detail1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.RemoveEventId)
              {
                this.game.Data.RemoveEvent(this.detail1);
                --this.detail1;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.AddCommandId)
              {
                this.game.Data.EventObj[this.detail1].AddCommand(this.detail2);
                this += 1.detail2;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.allowExecuteId)
              {
                this.game.Data.EventObj[this.detail1].AllowExecute = !this.game.Data.EventObj[this.detail1].AllowExecute;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.InsertId)
              {
                let mut index7: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give an event #", "Shadow Empire : Planetary Conquest")));
                let mut num10: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give start command line #", "Shadow Empire : Planetary Conquest")));
                let mut num11: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give end command line #", "Shadow Empire : Planetary Conquest"))) - num10 + 1;
                if (index7 > -1 & index7 <= this.game.Data.EventCounter)
                {
                  let mut num12: i32 =  num11;
                  for (let mut index8: i32 =  1; index8 <= num12; index8 += 1)
                  {
                    let mut index9: i32 =  num10 - 1 + index8;
                    if (this.game.Data.EventObj[index7].CommandCounter >= index9 & index9 > -1)
                    {
                      CommandClass commandClass = this.game.Data.EventObj[index7].CommandList[index9].Clone();
                      this.game.Data.EventObj[this.detail1].AddCommand(this.detail2);
                      this += 1.detail2;
                      if (index7 == this.detail1 & num10 > this.detail2)
                      {
                        num10 += 1;
                        num11 += 1;
                      }
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2] = commandClass;
                    }
                  }
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.CommandCopy)
              {
                this.game.EditObj.TempCommand = this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Clone();
                return windowReturnClass;
              }
              if (num1 == this.CommandPaste)
              {
                if (this.game.EditObj.TempCommand != null)
                {
                  this.game.Data.EventObj[this.detail1].AddCommand(this.detail2);
                  this += 1.detail2;
                  this.game.Data.EventObj[this.detail1].CommandList[this.detail2] = this.game.EditObj.TempCommand.Clone();
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.CommandUpId)
              {
                this.game.Data.EventObj[this.detail1].Commandup(this.detail2);
                --this.detail2;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.CommandDownId)
              {
                this.game.Data.EventObj[this.detail1].Commanddown(this.detail2);
                this += 1.detail2;
                if (this.detail2 > this.game.Data.EventObj[this.detail1].CommandCounter)
                  --this.detail2;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.RemoveCommandId)
              {
                this.game.Data.EventObj[this.detail1].RemoveCommand(this.detail2);
                --this.detail2;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.EventUpId)
              {
                this.game.Data.eventup(this.detail1);
                --this.detail1;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.EventDownId)
              {
                this.game.Data.eventdown(this.detail1);
                this += 1.detail1;
                if (this.detail1 > this.game.Data.EventCounter)
                  --this.detail1;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.info3id)
              {
                this.game.Data.EventObj[this.detail1].Name = Interaction.InputBox("Give Name for this Event", "Shadow Empire : Planetary Conquest");
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Typ1)
              {
                this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type = 1;
                let mut index10: i32 =  0;
                do
                {
                  let mut index11: i32 =  0;
                  do
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[index10, index11] = Conversions.ToString(0);
                    index11 += 1;
                  }
                  while (index11 <= 30);
                  index10 += 1;
                }
                while (index10 <= 2);
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Typ2)
              {
                this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type = 2;
                let mut index12: i32 =  0;
                do
                {
                  let mut index13: i32 =  0;
                  do
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[index12, index13] = Conversions.ToString(0);
                    index13 += 1;
                  }
                  while (index13 <= 30);
                  index12 += 1;
                }
                while (index12 <= 2);
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Typ3)
              {
                this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type = 3;
                let mut index14: i32 =  0;
                do
                {
                  let mut index15: i32 =  0;
                  do
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[index14, index15] = Conversions.ToString(0);
                    index15 += 1;
                  }
                  while (index15 <= 30);
                  index14 += 1;
                }
                while (index14 <= 2);
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Typ4)
              {
                this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type = 4;
                let mut index16: i32 =  0;
                do
                {
                  let mut index17: i32 =  0;
                  do
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[index16, index17] = Conversions.ToString(0);
                    index17 += 1;
                  }
                  while (index17 <= 30);
                  index16 += 1;
                }
                while (index16 <= 2);
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Typ5)
              {
                this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type = 5;
                let mut index18: i32 =  0;
                do
                {
                  let mut index19: i32 =  0;
                  do
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[index18, index19] = Conversions.ToString(0);
                    index19 += 1;
                  }
                  while (index19 <= 30);
                  index18 += 1;
                }
                while (index18 <= 2);
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Typ6)
              {
                this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type = 6;
                let mut index20: i32 =  0;
                do
                {
                  let mut index21: i32 =  0;
                  do
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[index20, index21] = Conversions.ToString(0);
                    index21 += 1;
                  }
                  while (index21 <= 30);
                  index20 += 1;
                }
                while (index20 <= 2);
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Typ7)
              {
                this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type = 7;
                let mut index22: i32 =  0;
                do
                {
                  let mut index23: i32 =  0;
                  do
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[index22, index23] = Conversions.ToString(0);
                    index23 += 1;
                  }
                  while (index23 <= 30);
                  index22 += 1;
                }
                while (index22 <= 2);
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.typstr)
              {
                if (this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type == 3)
                {
                  let mut integer: i32 =  Conversions.ToInteger(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 1]);
                  if (this.game.Data.ExecTypeString[integer] == 1)
                  {
                    this.DoingSlots = true;
                    this.SetSlot = 0;
                    this.StepCurrent = this.game.Data.ExecTypeVarCount[integer] + 1;
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (this.game.Data.ExecTypeString[integer] == 2)
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString = Interaction.InputBox("Give String", "Shadow Empire : Planetary Conquest", this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString);
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                if (this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type == 7)
                {
                  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString = Interaction.InputBox("Give String for Comment Line", "Shadow Empire : Planetary Conquest", this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num13: i32 =   Interaction.MsgBox( "Cannot edit tempstring. Is not exectype, or exectype has no text option.", Title: ( "Shadow Empire : Planetary Conquest"));
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.TypMode)
              {
                EventClass[] eventObj = this.game.Data.EventObj;
                EventClass[] eventClassArray = eventObj;
                let mut detail1: i32 =  this.detail1;
                let mut index24: i32 =  detail1;
                eventClassArray[index24].CheckMode = eventObj[detail1].CheckMode + 1;
                if (this.game.Data.EventObj[this.detail1].CheckMode > 12)
                  this.game.Data.EventObj[this.detail1].CheckMode = 0;
                this.bigclear();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.setPrio)
              {
                this.game.Data.EventObj[this.detail1].Priority =  Math.Round(Conversion.Val(Interaction.InputBox("Give new prio (0=default)", "Shadow Empire : Planetary Conquest", this.game.Data.EventObj[this.detail1].Priority.ToString())));
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Item1)
              {
                if (this.game.Data.EventObj[this.detail1].CommandList[this.detail2].type == 5)
                {
                  let mut num14: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give a TempVar between 0-999.", "Shadow Empire : Planetary Conquest")));
                  if (num14 < 0 | num14 > 999)
                  {
                    let mut num15: i32 =   Interaction.MsgBox( "between 0 and 999 please", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 0] = Conversions.ToString(num14);
                    this.bigclear();
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                  }
                }
                else
                {
                  this.DoingSlots = true;
                  this.SetSlot = 0;
                  this.StepCurrent = 0;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
              else
              {
                if (num1 == this.Item2)
                {
                  this.DoingSlots = true;
                  this.SetSlot = 1;
                  this.StepCurrent = 0;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.Item3)
                {
                  this.DoingSlots = true;
                  this.SetSlot = 2;
                  this.StepCurrent = 0;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.tabx1)
                {
                  this.DoingSlots = false;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.tabx2)
                {
                  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(0);
                  --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.quick1)
                {
                  this.DoingSlots = true;
                  this.DoEscape = true;
                  this.SetSlot = 0;
                  this.StepCurrent = 2;
                  if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 0]) == 4.0)
                    --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.quick2)
                {
                  this.DoingSlots = true;
                  this.DoEscape = true;
                  this.SetSlot = 0;
                  this.StepCurrent = 3;
                  if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 0]) == 4.0)
                    --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.quick3)
                {
                  this.DoingSlots = true;
                  this.DoEscape = true;
                  this.SetSlot = 0;
                  this.StepCurrent = 4;
                  if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 0]) == 4.0)
                    --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.quick4)
                {
                  this.DoingSlots = true;
                  this.DoEscape = true;
                  this.SetSlot = 0;
                  this.StepCurrent = 5;
                  if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[0, 0]) == 4.0)
                    --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.quick5)
                {
                  this.DoingSlots = true;
                  this.DoEscape = true;
                  this.SetSlot = 2;
                  this.StepCurrent = 2;
                  if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[2, 0]) == 4.0)
                    --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.quick6)
                {
                  this.DoingSlots = true;
                  this.DoEscape = true;
                  this.SetSlot = 2;
                  this.StepCurrent = 3;
                  if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[2, 0]) == 4.0)
                    --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.quick7)
                {
                  this.DoingSlots = true;
                  this.DoEscape = true;
                  this.SetSlot = 2;
                  this.StepCurrent = 4;
                  if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[2, 0]) == 4.0)
                    --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.quick8)
                {
                  this.DoingSlots = true;
                  this.DoEscape = true;
                  this.SetSlot = 2;
                  this.StepCurrent = 5;
                  if (Conversions.ToDouble(this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[2, 0]) == 4.0)
                    --this.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.OptionsListId)
                {
                  let mut num16: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  if (num16 != -1)
                  {
                    this.detail1 = num16;
                    this.dostuff();
                  }
                  else
                    this.SubPartFlag[index1] = true;
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                num17: i32;
                if (num1 == this.DescriptId)
                {
                  num17 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.OptionsList5Id)
                {
                  let mut num18: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  if (this.cat != num18)
                  {
                    this.cat = num18;
                    this.dostuff();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.OptionsList6Id)
                {
                  let mut num19: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  if (num19 > 0)
                    --num19;
                  if (this.detailLib != num19)
                  {
                    this.detailLib = num19;
                    this.dostuff();
                  }
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.tta)
                {
                  num17 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  this.SubPartFlag[index1] = true;
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.OptionsList2Id)
                {
                  let mut num20: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  if (num20 != -1)
                  {
                    this.detail2 = num20;
                    this.dostuff();
                  }
                  else
                    this.SubPartFlag[index1] = true;
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.OptionsList3Id)
                {
                  this.detail3 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.GroupListId)
                {
                  this.groupdetail =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.OptionsList4Id)
                {
                  this.detail4 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.Tab1)
                {
                  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                  this += 1.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.Tab2)
                {
                  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(2);
                  this += 1.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (num1 == this.Tab6)
                {
                  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                  this += 1.StepCurrent;
                  this.dostuff();
                  let mut num21: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give a TempVar number between 0 and 999", "Shadow Empire : Planetary Conquest")));
                  if (num21 < 0 | num21 > 999)
                  {
                    let mut num22: i32 =   Interaction.MsgBox( "Beep. Stay in the limits plz", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(1);
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(num21);
                    this += 1.StepCurrent;
                    if (this.DoEscape)
                    {
                      this.DoEscape = false;
                      this.DoingSlots = false;
                    }
                    this.bigclear();
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else if (num1 == this.Tab9)
                {
                  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                  this += 1.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  let mut num23: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give a Real number between -999999 and 999999", "Shadow Empire : Planetary Conquest")));
                  if (num23 < -999999 | num23 > 999999)
                  {
                    let mut num24: i32 =   Interaction.MsgBox( "Beep. Stay in the limits plz", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(4);
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(num23);
                    this += 1.StepCurrent;
                    if (this.DoEscape)
                    {
                      this.DoEscape = false;
                      this.DoingSlots = false;
                    }
                    this.bigclear();
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else if (num1 == this.Tab10)
                {
                  this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                  this += 1.StepCurrent;
                  this.bigclear();
                  this.dostuff();
                  let mut num25: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give a TempString number between 0 and 999", "Shadow Empire : Planetary Conquest")));
                  if (num25 < -999999 | num25 > 999999)
                  {
                    let mut num26: i32 =   Interaction.MsgBox( "Beep. Stay in the limits plz", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(7);
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(num25);
                    this += 1.StepCurrent;
                    if (this.DoEscape)
                    {
                      this.DoEscape = false;
                      this.DoingSlots = false;
                    }
                    this.bigclear();
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else if (num1 == this.Tab10b)
                {
                  let mut num27: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give a TempString number between 0 and 999", "Shadow Empire : Planetary Conquest")));
                  if (num27 < 0 | num27 > 999)
                  {
                    let mut num28: i32 =   Interaction.MsgBox( "Beep. Stay in the limits plz", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(7);
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(num27);
                    this += 1.StepCurrent;
                    if (this.DoEscape)
                    {
                      this.DoEscape = false;
                      this.DoingSlots = false;
                    }
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                else
                {
                  if (num1 == this.tab19)
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                    this += 1.StepCurrent;
                    this.bigclear();
                    this.dostuff();
                    str: String = Interaction.InputBox("Give a Real String", "Shadow Empire : Planetary Conquest", this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra]);
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(8);
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = str;
                    this += 1.StepCurrent;
                    if (this.DoEscape)
                    {
                      this.DoEscape = false;
                      this.DoingSlots = false;
                    }
                    this.bigclear();
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.Tab7)
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                    this += 1.StepCurrent;
                    this.bigclear();
                    this.dostuff();
                    this.detailchoice = 2;
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.Tab8)
                  {
                    this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                    this += 1.StepCurrent;
                    this.bigclear();
                    this.dostuff();
                    this.detailchoice = 3;
                    this.dostuff();
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  if (num1 == this.Tab6b)
                  {
                    let mut num29: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give a TempVar number between 0 and 999", "Shadow Empire : Planetary Conquest")));
                    if (num29 < 0 | num29 > 999)
                    {
                      let mut num30: i32 =   Interaction.MsgBox( "Beep. Stay in the limits plz", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(1);
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(num29);
                      this += 1.StepCurrent;
                      if (this.DoEscape)
                      {
                        this.DoEscape = false;
                        this.DoingSlots = false;
                      }
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                  }
                  else if (num1 == this.Tab9b)
                  {
                    let mut num31: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give a Real number between -9999 and 9999", "Shadow Empire : Planetary Conquest")));
                    if (num31 < -999999 | num31 > 999999)
                    {
                      let mut num32: i32 =   Interaction.MsgBox( "Beep. Stay in the limits plz", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(4);
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(num31);
                      this += 1.StepCurrent;
                      if (this.DoEscape)
                      {
                        this.DoEscape = false;
                        this.DoingSlots = false;
                      }
                      this.bigclear();
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                  }
                  else
                  {
                    if (num1 == this.tab19b)
                    {
                      str: String = Interaction.InputBox("Give a Real String", "Shadow Empire : Planetary Conquest", this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra]);
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(8);
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = str;
                      this += 1.StepCurrent;
                      if (this.DoEscape)
                      {
                        this.DoEscape = false;
                        this.DoingSlots = false;
                      }
                      this.bigclear();
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.Tab7b)
                    {
                      this.detailchoice = 2;
                      this.bigclear();
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.Tab8b)
                    {
                      this.detailchoice = 3;
                      this.bigclear();
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.NTab1)
                    {
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                      this += 1.StepCurrent;
                      this.bigclear();
                      this.dostuff();
                      this.detailchoice = 5;
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.ntab1x)
                    {
                      this.detailchoice = 5;
                      this.bigclear();
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.ntab2x)
                    {
                      this.detailchoice = 6;
                      this.bigclear();
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.NTab2)
                    {
                      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 0] = Conversions.ToString(1);
                      this += 1.StepCurrent;
                      this.bigclear();
                      this.dostuff();
                      this.detailchoice = 6;
                      this.dostuff();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (num1 == this.Tab5)
                    {
                      if (this.detailchoice == 2)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(2);
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(this.detail4);
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 3 + this.VarExtra] = Conversions.ToString(this.detail3);
                        this += 1.StepCurrent;
                        if (this.DoEscape)
                        {
                          this.DoEscape = false;
                          this.DoingSlots = false;
                        }
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (this.detailchoice == 3)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(3);
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(this.detail3);
                        this += 1.StepCurrent;
                        if (this.DoEscape)
                        {
                          this.DoEscape = false;
                          this.DoingSlots = false;
                        }
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (this.detailchoice == 5)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(5);
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(this.game.Data.HistoricalUnitObj[this.detail3].ID);
                        this += 1.StepCurrent;
                        if (this.DoEscape)
                        {
                          this.DoEscape = false;
                          this.DoingSlots = false;
                        }
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (this.detailchoice == 6)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1 + this.VarExtra] = Conversions.ToString(6);
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 2 + this.VarExtra] = Conversions.ToString(this.game.Data.AreaObj[this.detail3].ID);
                        this += 1.StepCurrent;
                        if (this.DoEscape)
                        {
                          this.DoEscape = false;
                          this.DoingSlots = false;
                        }
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                    }
                    else
                    {
                      if (num1 == this.tab11)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(1);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab12)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(2);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab13)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(3);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab131)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(4);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab132)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(5);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab133)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(6);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.Tab3)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(this.detail3);
                        this.bigclear();
                        this += 1.StepCurrent;
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.Tab4)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(this.detail3);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab14)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(1);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab15)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(2);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab16)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(3);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab17)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(4);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tab18)
                      {
                        this.game.Data.EventObj[this.detail1].CommandList[this.detail2].Data[this.SetSlot, 1] = Conversions.ToString(5);
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tok)
                      {
                        this += 1.StepCurrent;
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.tok2)
                      {
                        Form2::new( this.formref).Initialize(this.game.Data, 3, this.detail1, this.detail2);
                        this.bigclear();
                        this.dostuff();
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                      if (num1 == this.Export)
                      {
                        this.game.FormRef.Cursor = Cursors.WaitCursor;
                        this.DoExport();
                        this.game.FormRef.Cursor = Cursors.Default;
                        let mut num33: i32 =   Interaction.MsgBox( "Done");
                        windowReturnClass.SetFlag(true);
                        return windowReturnClass;
                      }
                    }
                  }
                }
              }
            }
            let mut index25: i32 =  0;
            do
            {
              let mut num34: i32 =  this.SubPartID[index1];
              if (num34 == this.tbutton[index25])
              {
                this.temptext[index25] = Interaction.InputBox("Give Text Line.... (dont use #,@,*,)", "Shadow Empire : Planetary Conquest");
                this.ReconstructString();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num34 == this.toption[index25])
              {
                if (this.tempoption[index25] == 1)
                {
                  this.tempoption[index25] = 0;
                }
                else
                {
                  let mut num35: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Leads to which event nr#??", "Shadow Empire : Planetary Conquest")));
                  if (num35 < 0 | num35 > this.game.Data.EventCounter)
                  {
                    let mut num36: i32 =   Interaction.MsgBox( "No valid event#", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    this.tempoption[index25] = 1;
                    this.temprefer[index25] = num35;
                  }
                }
                this.ReconstructString();
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              index25 += 1;
            }
            while (index25 <= 19);
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn DoExport()
    {
      let mut eventCounter: i32 =  this.game.Data.EventCounter;
      for (let mut enr: i32 =  0; enr <= eventCounter; enr += 1)
      {
        StreamWriter text = File.CreateText(this.game.AppPath + "logs/" + this.game.Data.Name + "_eventID" + Conversion.Str( this.game.Data.EventObj[enr].Id) + ".txt");
        let mut num: i32 =  0;
        if (this.game.Data.EventObj[enr].CommandCounter > -1)
        {
          let mut commandCounter: i32 =  this.game.Data.EventObj[enr].CommandCounter;
          for (let mut index: i32 =  0; index <= commandCounter; index += 1)
          {
            str1: String = this.COMMANDTYPE[this.game.Data.EventObj[enr].CommandList[index].type];
            if (this.game.Data.EventObj[enr].CommandList[index].type == 2)
            {
              --num;
              if (0 > num)
                num = 0;
            }
            if (this.game.Data.EventObj[enr].CommandList[index].type == 6)
            {
              --num;
              if (0 > num)
                num = 0;
            }
            if (this.game.Data.EventObj[enr].CommandList[index].type == 1)
              str1 = str1 + ": " + this.ItemInfo(enr, index, 0) + " " + this.ItemInfo(enr, index, 1) + " " + this.ItemInfo(enr, index, 2);
            if (this.game.Data.EventObj[enr].CommandList[index].type == 3)
              str1 = str1 + ": " + this.ItemInfo(enr, index, 0);
            if (this.game.Data.EventObj[enr].CommandList[index].type == 7)
              str1 = "' " + this.game.Data.EventObj[enr].CommandList[index].DataString;
            if (this.game.Data.EventObj[enr].CommandList[index].type == 4)
              str1 = str1 + ": " + this.ItemInfo(enr, index, 0) + " " + this.ItemInfo(enr, index, 1) + " " + this.ItemInfo(enr, index, 2);
            if (this.game.Data.EventObj[enr].CommandList[index].type == 5)
              str1 = str1 + ": " + this.ItemInfo(enr, index, 0) + " " + this.ItemInfo(enr, index, 1) + " " + this.ItemInfo(enr, index, 2);
            str2: String = Strings.Trim(Conversion.Str( index)) + ") " + Strings.Space(4 - Strings.Len(Strings.Trim(Conversion.Str( index)))) + Strings.Space(num * 2) + str1;
            text.WriteLine(str2);
            if (this.game.Data.EventObj[enr].CommandList[index].type == 1)
              num += 1;
            if (this.game.Data.EventObj[enr].CommandList[index].type == 5)
              num += 1;
          }
        }
        text.Close();
      }
    }

    pub fn ReconstructString()
    {
      let mut num1: i32 =  -1;
      str: String = "";
      let mut index1: i32 =  0;
      do
      {
        if (Strings.Len(this.temptext[index1]) > 0)
          num1 = index1;
        index1 += 1;
      }
      while (index1 <= 19);
      let mut num2: i32 =  num1;
      for (let mut index2: i32 =  0; index2 <= num2; index2 += 1)
      {
        if (this.tempoption[index2] == 1)
          str += "*";
        str += this.temptext[index2];
        if (this.tempoption[index2] == 1)
          str = str + "@" + Strings.Trim(Conversion.Str( this.temprefer[index2]));
        if (index2 < num1)
          str += "#";
      }
      this.game.Data.EventObj[this.detail1].CommandList[this.detail2].DataString = str;
    }
  }
}
